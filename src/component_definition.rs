mod surfaces;

use crate::{
    domtree::{Document, Element, HasAttr, HasAttrMut, error::AttrError},
    helpers::List,
};
use quick_xml::Reader;
use std::{fmt::Display, io::BufRead, path::Path, str::FromStr};
pub use surfaces::{Surface, SurfaceOrientation};

#[derive(Clone, Debug)]
pub struct ComponentDefinition {
    tree: Document,
}

impl ComponentDefinition {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, quick_xml::Error> {
        let mut reader = Reader::from_file(path)?;
        Self::from_reader(&mut reader)
    }

    fn from_reader<R: BufRead>(reader: &mut Reader<R>) -> Result<Self, quick_xml::Error> {
        let tree = Document::from_xml_reader(reader)?;
        Ok(Self { tree })
    }

    fn attr<K: AsRef<[u8]>, T, E>(&self, key: K) -> Option<T>
    where
        T: FromStr<Err = E>,
        AttrError: From<E>,
    {
        self.tree
            .find(&["definition"])
            .and_then(|el| el.attr(key).ok())
    }
    fn set_attr<K: AsRef<[u8]>, T: Display>(&mut self, key: K, value: T) {
        self.tree
            .find_ensure("definition", &[])
            .set_attr(key, value);
    }

    pub fn name(&self) -> Option<String> {
        self.attr("name")
    }
    pub fn set_name(&mut self, value: String) {
        self.set_attr("name", value);
    }

    pub fn surfaces(&self) -> Option<List<&Element, Surface<&Element>>> {
        self.tree.find(&["definition", "surfaces"]).map(List::new)
    }
    pub fn surfaces_mut(&mut self) -> List<&mut Element, Surface<&mut Element>> {
        let surfaces = self.tree.find_ensure("definitions", &["surfaces"]);
        List::new(surfaces)
    }

    pub fn buoyancy_surfaces(&self) -> Option<List<&Element, Surface<&Element>>> {
        self.tree
            .find(&["definition", "buoyancy_surfaces"])
            .map(List::new)
    }
    pub fn buoyancy_surfaces_mut(&mut self) -> List<&mut Element, Surface<&mut Element>> {
        let surfaces = self.tree.find_ensure("definitions", &["buoyancy_surfaces"]);
        List::new(surfaces)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read() {
        let definitions_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("test_data")
            .join("vanilla_definitions");
        let items = [
            ("01_block.xml", "Block"),
            ("02_wedge.xml", "Wedge"),
            ("03_pyramid.xml", "Pyramid"),
            ("04_invpyramid.xml", "Inverse Pyramid"),
        ];

        for (filename, name) in items {
            let definition = ComponentDefinition::load(definitions_dir.join(filename))
                .expect("failed to load {filename:?}");

            assert_eq!(definition.name(), Some(name.to_owned()));

            if let Some(surfaces) = definition.surfaces() {
                for surface in surfaces.iter() {
                    assert!(surface.orientation().is_ok());
                    assert!(surface.rotation().is_ok());
                    assert!(surface.shape().is_ok());
                    assert!(surface.trans_type().is_ok());
                }
            }
            if let Some(surfaces) = definition.buoyancy_surfaces() {
                for surface in surfaces.iter() {
                    assert!(surface.orientation().is_ok());
                    assert!(surface.rotation().is_ok());
                    assert!(surface.shape().is_ok());
                    assert!(surface.trans_type().is_ok());
                }
            }
        }
    }

    #[test]
    fn vanilla_definitions() {
        let definitions_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("test_data")
            .join("vanilla_definitions");

        let mut xml_paths = Vec::new();
        for entry in std::fs::read_dir(&definitions_dir)
            .unwrap_or_else(|e| panic!("failed to read dir {definitions_dir:?}: {e}"))
        {
            let entry = entry
                .unwrap_or_else(|e| panic!("failed to read entry in {definitions_dir:?}: {e}"));
            let path = entry.path();
            if path
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("xml"))
            {
                xml_paths.push(path);
            }
        }

        assert!(
            !xml_paths.is_empty(),
            "no .xml files found under {definitions_dir:?}"
        );

        use std::sync::{
            Mutex,
            atomic::{AtomicBool, AtomicUsize, Ordering},
        };

        let worker_count = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1)
            .min(xml_paths.len());

        let next_index = AtomicUsize::new(0);
        let failed = AtomicBool::new(false);
        let first_error: Mutex<Option<(std::path::PathBuf, String)>> = Mutex::new(None);

        std::thread::scope(|scope| {
            for _ in 0..worker_count {
                scope.spawn(|| {
                    loop {
                        if failed.load(Ordering::Relaxed) {
                            break;
                        }

                        let idx = next_index.fetch_add(1, Ordering::Relaxed);
                        if idx >= xml_paths.len() {
                            break;
                        }

                        let path = &xml_paths[idx];
                        if let Err(e) = ComponentDefinition::load(path) {
                            failed.store(true, Ordering::Relaxed);
                            let mut guard = first_error.lock().expect("mutex poisoned");
                            if guard.is_none() {
                                *guard = Some((path.clone(), e.to_string()));
                            }
                            break;
                        }
                    }
                });
            }
        });

        if let Some((path, err)) = first_error.into_inner().expect("mutex poisoned") {
            panic!("ComponentDefinition::load({path:?}) failed: {err}");
        }
    }
}

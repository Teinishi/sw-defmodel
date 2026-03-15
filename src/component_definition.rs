mod definition;
mod surfaces;

use crate::{
    domtree::{Document, Element, HasChildren, HasChildrenMut},
    helpers::ListItem,
};
pub use definition::Definition;
use quick_xml::Reader;
use std::{io::BufRead, path::Path};
pub use surfaces::{Surface, SurfaceOrientation};

#[derive(Clone, Debug)]
pub struct ComponentDefinition {
    tree: Document,
}

impl ComponentDefinition {
    pub fn from_xml_str(s: &str) -> Result<Self, quick_xml::Error> {
        Ok(Self {
            tree: Document::from_xml_str(s)?,
        })
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, quick_xml::Error> {
        Ok(Self {
            tree: Document::from_file(path)?,
        })
    }

    pub fn from_xml_reader<R: BufRead>(reader: &mut Reader<R>) -> Result<Self, quick_xml::Error> {
        Ok(Self {
            tree: Document::from_xml_reader(reader)?,
        })
    }

    pub fn write<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.tree.write(writer)
    }

    pub fn to_bytes(&self) -> std::io::Result<Vec<u8>> {
        self.tree.to_bytes()
    }

    pub fn definition(&self) -> Option<Definition<&Element>> {
        self.tree
            .single_element_by_name(Definition::<&Element>::NAME)
            .map(|(el, _)| Definition::from_element(el))
    }

    pub fn definition_mut(&mut self) -> Definition<&mut Element> {
        let (el, _) = self.tree.ensure_element(Definition::<&mut Element>::NAME);
        Definition::from_element(el)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mutation_test_helper<F>(input: &str, expected: &str, callback: F)
    where
        F: FnOnce(&mut Definition<&mut Element>),
    {
        let mut cd = ComponentDefinition::from_xml_str(input).expect("failed to parse");
        let mut definition = cd.definition_mut();

        callback(&mut definition);

        let out = cd.to_bytes().expect("write failed");
        if out != expected.as_bytes() {
            panic!(
                "assertion `out == expected` failed\nout:\n{}\nexpected:\n{}",
                crate::utils::debug_utf8(&out),
                expected
            );
        }
        assert_eq!(out, expected.as_bytes());
    }

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
            let cd = ComponentDefinition::from_file(definitions_dir.join(filename))
                .expect("failed to load {filename:?}");
            let definition = cd.definition().expect("failed to get <definition>");

            assert_eq!(definition.name(), Ok(name.to_owned()));
            definition.category().expect("failed to get category");

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
    fn list_mutation() {
        mutation_test_helper(
            concat!(
                "<definition>\n",
                "  <surfaces>\n",
                "    <surface orientation=\"0\" />\n",
                "    <surface orientation=\"1\" />\n",
                "    <surface orientation=\"2\" />\n",
                "    <surface orientation=\"3\" />\n",
                "    <surface orientation=\"4\" />\n",
                "    <surface orientation=\"5\" />\n",
                "  </surfaces>\n",
                "</definition>\n"
            ),
            concat!(
                "<definition>\n",
                "  <surfaces>\n",
                "    <surface orientation=\"5\" />\n",
                "    <surface orientation=\"4\" />\n",
                "    <surface orientation=\"3\" />\n",
                "    <surface orientation=\"2\" />\n",
                "    <surface orientation=\"1\" />\n",
                "    <surface orientation=\"0\" />\n",
                "  </surfaces>\n",
                "</definition>\n"
            ),
            |definition| {
                let mut surfaces = definition.surfaces_mut();
                for mut surface in surfaces.iter_mut() {
                    let new_orientation = match surface
                        .orientation()
                        .expect("failed to get orientation")
                    {
                        SurfaceOrientation::XPos => SurfaceOrientation::ZNeg,
                        SurfaceOrientation::XNeg => SurfaceOrientation::ZPos,
                        SurfaceOrientation::YPos => SurfaceOrientation::YNeg,
                        SurfaceOrientation::YNeg => SurfaceOrientation::YPos,
                        SurfaceOrientation::ZPos => SurfaceOrientation::XNeg,
                        SurfaceOrientation::ZNeg => SurfaceOrientation::XPos,
                        SurfaceOrientation::Unknown(_) => panic!("unexpected surface orientation"),
                    };
                    surface.set_orientation(new_orientation);
                }
            },
        );
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
                        if let Err(e) = ComponentDefinition::from_file(path) {
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

use super::{
    domtree::Document,
    domtree::{Element, HasChildren},
};
use quick_xml::Reader;
use std::{io::BufRead, path::Path};

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

    fn definition_el(&self) -> Option<&Element> {
        self.tree
            .single_element_by_name("definition")
            .map(|(el, _)| el)
    }
    fn definition_el_ensure(&mut self) -> &mut Element {
        self.tree.ensure_element("definition").0
    }

    pub fn name(&self) -> Option<String> {
        self.definition_el().and_then(|el| el.attr("name").ok())
    }
    pub fn set_name(&mut self, value: String) {
        self.definition_el_ensure().set_attr("name", value);
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

#![recursion_limit = "512"]
#[macro_use]
pub(crate) mod macros;

pub mod definition_view;
pub mod domtree;
pub mod generic_view;
pub mod helpers;
pub(crate) mod utils;
pub mod vehicle_view;

pub use definition_view::DefinitionDocument;
pub use vehicle_view::VehicleDocument;

#[cfg(test)]
mod tests {
    use crate::definition_view::{Definition, DefinitionDocument};
    use crate::domtree::Element;
    use std::path::Path;

    fn mutation_test_helper<F>(input: &str, expected: &str, callback: F)
    where
        F: FnOnce(&mut Definition<&mut Element>),
    {
        let mut cd = DefinitionDocument::from_xml_str(input).expect("failed to parse");
        let mut definition = cd.root_mut();

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
    fn read_basic_block() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("test_data")
            .join("vanilla_definitions")
            .join("01_block.xml");

        let cd = DefinitionDocument::from_file(path).expect("failed to load {path:?}");
        let definition = cd.root().expect("failed to get <definition>");

        assert_eq!(definition.name(), Ok("Block".to_owned()));
        assert_eq!(definition.category(), Ok(0));
        assert_eq!(definition.type_attr(), Ok(0));
        assert_eq!(definition.mass(), Ok(1.0));
        assert_eq!(definition.value(), Ok(2));
        assert_eq!(definition.flags(), Ok(56));
        assert_eq!(definition.tags(), Ok("basic".to_owned()));
        assert_eq!(
            definition.voxel_min().map(|v| (v.x(), v.y(), v.z())),
            Some((Ok(0), Ok(0), Ok(0)))
        );

        assert_eq!(
            definition
                .surfaces()
                .expect("failed to get <surfaces>")
                .len(),
            6
        );
        assert_eq!(
            definition
                .buoyancy_surfaces()
                .expect("failed to get <buoyancy_surfaces>")
                .len(),
            6
        );
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
                    let orientation = surface.orientation().expect("failed to get orientation");
                    surface.set_orientation((5 - orientation).into());
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
                        if let Err(e) = DefinitionDocument::from_file(path) {
                            failed.store(true, Ordering::Relaxed);
                            let mut guard = first_error.lock().expect("mutex poisoned");
                            if guard.is_none() {
                                *guard = Some((path.clone(), format!("{:?}", e)));
                            }
                            break;
                        }
                    }
                });
            }
        });

        if let Some((path, err)) = first_error.into_inner().expect("mutex poisoned") {
            panic!("DefinitionDocument::load({path:?}) failed: {err}");
        }
    }
}

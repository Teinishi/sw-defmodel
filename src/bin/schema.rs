// サンプルXMLを読んでスキーマ生成

use std::path::Path;
use sw_defmodel::domtree::{Document, Element, HasChildren};

#[derive(Default, Debug)]
struct Schema {}

impl Schema {
    pub fn update(&mut self, root: &Element) {}
}

fn main() {
    let definitions_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("test_data")
        .join("vanilla_definitions");

    let mut schema = Schema::default();

    for entry in std::fs::read_dir(&definitions_dir)
        .unwrap_or_else(|e| panic!("failed to read dir {definitions_dir:?}: {e}"))
    {
        let entry =
            entry.unwrap_or_else(|e| panic!("failed to read entry in {definitions_dir:?}: {e}"));
        let path = entry.path();
        if path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("xml"))
        {
            let doc = Document::from_file(path).expect("failed to read {path:?}");
            schema.update(doc.single_element_by_name("definition").expect("failed to get <definition> in {path:?}").0);
        }
    }
}

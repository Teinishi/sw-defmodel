use super::schema_analyzer::{SchemaAttribute, SchemaChild};
use std::io;

#[derive(Debug)]
pub(super) enum ChildElementType {
    NamedUnique(&'static str),
    #[expect(dead_code)]
    Unique,
    #[expect(dead_code)]
    List,
}

pub(super) trait SchemaWriteRule {
    #[expect(unused_variables)]
    fn before_define_attribute(
        &mut self,
        tag_name: &str,
        attribute: &SchemaAttribute,
    ) -> Option<String> {
        None
    }

    #[expect(unused_variables)]
    fn before_scan_child(
        &mut self,
        tag_name: &str,
        child: &SchemaChild,
    ) -> Option<ChildElementType> {
        None
    }

    #[expect(unused_variables)]
    fn finalize<W: io::Write>(
        &mut self,
        f: &mut io::BufWriter<W>,
        tag_name: &str,
    ) -> io::Result<()> {
        Ok(())
    }
}

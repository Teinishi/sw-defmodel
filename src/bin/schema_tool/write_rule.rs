use super::{
    enums::{ChildElementType, ValueType},
    schema_analyzer::{SchemaAttribute, SchemaChild},
};
use std::io;

pub(super) trait SchemaWriteRule {
    const MAX_ENUM: usize;

    #[expect(unused_variables)]
    fn before_define_attribute(
        &mut self,
        tag_name: &str,
        attribute: &SchemaAttribute,
    ) -> Option<ValueType> {
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

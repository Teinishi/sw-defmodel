use super::{
    enums::{ChildElementType, PrimitiveType, ValueType},
    schema_analyzer::{SchemaAttribute, SchemaChild},
};
use std::io;

pub(super) trait SchemaWriteRule {
    const MAX_ENUM: usize;
    const TARGET_LABEL: &str;

    #[expect(unused_variables)]
    fn before_define_attribute(
        &mut self,
        tag_name: &str,
        attribute: &SchemaAttribute,
    ) -> OverrideAttribute {
        Default::default()
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
        f: &mut W,
        tag_name: &str,
        items: &mut Vec<String>,
    ) -> io::Result<()> {
        Ok(())
    }
}

#[derive(Default, Debug)]
pub(super) struct OverrideAttribute {
    pub(super) doc: Option<&'static str>,
    pub(super) val_type: Option<ValueType>,
}

impl OverrideAttribute {
    #[allow(dead_code)]
    pub(super) fn primitive(doc: Option<&'static str>, prim: PrimitiveType) -> Self {
        Self {
            doc,
            val_type: Some(ValueType::Primitive(prim)),
        }
    }

    #[allow(dead_code)]
    pub(super) fn enum_u32(
        doc: Option<&'static str>,
        name: &str,
        variants: &[(&str, u32)],
    ) -> Self {
        Self {
            doc,
            val_type: Some(ValueType::EnumU32 {
                name: name.to_owned(),
                variants: variants
                    .iter()
                    .map(|(vn, vv)| (vn.to_string(), *vv))
                    .collect(),
                doc: None,
            }),
        }
    }

    #[allow(dead_code)]
    pub(super) fn enum_u64(
        doc: Option<&'static str>,
        name: &str,
        variants: &[(&str, u64)],
    ) -> Self {
        Self {
            doc,
            val_type: Some(ValueType::EnumU64 {
                name: name.to_owned(),
                variants: variants
                    .iter()
                    .map(|(vn, vv)| (vn.to_string(), *vv))
                    .collect(),
                doc: None,
            }),
        }
    }

    #[allow(dead_code)]
    pub(super) fn enum_i32(
        doc: Option<&'static str>,
        name: &str,
        variants: &[(&str, i32)],
    ) -> Self {
        Self {
            doc,
            val_type: Some(ValueType::EnumI32 {
                name: name.to_owned(),
                variants: variants
                    .iter()
                    .map(|(vn, vv)| (vn.to_string(), *vv))
                    .collect(),
                doc: None,
            }),
        }
    }

    #[allow(dead_code)]
    pub(super) fn enum_str(
        doc: Option<&'static str>,
        name: &str,
        variants: &[(&str, &str)],
    ) -> Self {
        Self {
            doc,
            val_type: Some(ValueType::EnumString {
                name: name.to_owned(),
                variants: variants
                    .iter()
                    .map(|(vn, vv)| (vn.to_string(), vv.to_string()))
                    .collect(),
                doc: None,
            }),
        }
    }
}

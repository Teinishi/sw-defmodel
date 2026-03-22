use super::node_info::ChildInfo;
use std::borrow::Cow;

pub(super) trait CodeRule {
    const TARGET_LABEL: &str;

    #[allow(unused)]
    fn override_child(&mut self, name: &str, info: &ChildInfo) -> Option<ChildClassificcation> {
        None
    }

    #[allow(unused)]
    fn override_child_type(&mut self, name: &str, info: &ChildInfo) -> Option<Cow<'static, str>> {
        None
    }
}

#[derive(Debug)]
pub(super) enum ChildClassificcation {
    Unique {
        name: Option<String>,
        ty: Option<TypeDefinition>,
    },
    List {
        list_name: Option<String>,
        item_ty: Option<TypeDefinition>,
    },
}

impl ChildClassificcation {
    pub(super) fn unique() -> Self {
        Self::Unique {
            name: None,
            ty: None,
        }
    }

    pub(super) fn unique_inline(ty: &'static str) -> Self {
        Self::Unique {
            name: None,
            ty: Some(TypeDefinition::Inline(ty)),
        }
    }

    pub(super) fn list() -> Self {
        Self::List {
            list_name: None,
            item_ty: None,
        }
    }
}

#[derive(Debug)]
pub(super) enum TypeDefinition {
    Inline(&'static str),
    Registered(usize),
}

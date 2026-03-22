use super::node_info::ChildInfo;
use std::io;

pub(super) trait CodeRule {
    const TARGET_LABEL: &str;

    #[allow(unused_variables)]
    fn override_child(
        &mut self,
        path: &NamePath,
        info: &ChildInfo,
    ) -> Option<ChildClassificcation> {
        None
    }

    #[allow(unused_variables)]
    fn finalize<W: io::Write>(&mut self, f: &mut W) -> io::Result<()> {
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub(super) struct NamePath {
    path: Vec<String>,
}

impl NamePath {
    pub(super) fn new(name: String) -> Self {
        Self { path: vec![name] }
    }

    pub(super) fn join(&self, value: String) -> Self {
        let mut s = self.clone();
        s.path.push(value);
        s
    }

    pub(super) fn name(&self) -> &str {
        self.path.last().unwrap()
    }

    pub(super) fn len(&self) -> usize {
        self.path.len()
    }

    #[expect(dead_code)]
    pub(super) fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub(super) fn tail(&self, size: usize) -> &[String] {
        &self.path[self.len().saturating_sub(size)..]
    }

    pub(super) fn join_str(&self, sep: &str) -> String {
        self.path.join(sep)
    }
}

impl std::fmt::Display for NamePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.path.join("/"))
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

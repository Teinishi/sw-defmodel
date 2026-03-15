use super::domtree::{Element, HasChildren, error::AttrError};
use strum::{Display, EnumString};

#[derive(PartialEq, Eq, Clone, Copy, Debug, Display, EnumString)]
pub enum SurfaceOrientation {
    #[strum(serialize = "0")]
    XPos,
    #[strum(serialize = "1")]
    XNeg,
    #[strum(serialize = "2")]
    YPos,
    #[strum(serialize = "3")]
    YNeg,
    #[strum(serialize = "4")]
    ZPos,
    #[strum(serialize = "5")]
    ZNeg,
}

#[derive(Debug)]
pub struct Surface<'a> {
    pub(crate) element: &'a Element,
}

impl<'a> Surface<'a> {
    pub fn orientation(&self) -> Result<SurfaceOrientation, AttrError> {
        self.element.attr("orientation")
    }
}

#[derive(Debug)]
pub struct SurfaceList<'a> {
    pub(crate) element: &'a Element,
}

impl<'a> SurfaceList<'a> {
    pub fn len(&self) -> usize {
        self.element.elements_by_name("surface").count()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = Surface<'a>> {
        self.element
            .elements_by_name("surface")
            .map(|e| Surface { element: e.0 })
    }
}

#[derive(Debug)]
pub struct SurfaceListMut<'a> {
    pub(crate) element: &'a mut Element,
}

use crate::domtree::{Element, HasChildren, HasChildrenMut};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct List<E, T> {
    pub(crate) element: E,
    pub(crate) item_name: &'static str,
    _marker: PhantomData<T>,
}

impl<E, T> List<E, T> {
    pub fn new(element: E, item_name: &'static str) -> Self {
        Self {
            element,
            item_name,
            _marker: PhantomData,
        }
    }
}

impl<E: HasChildren, T> List<E, T> {
    pub fn len(&self) -> usize
    {
        self.element.elements_by_name(self.item_name).count()
    }

    pub fn is_empty(&self) -> bool
    {
        self.len() == 0
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = T>
    where
        T: From<&'a Element>,
    {
        self.element
            .elements_by_name(self.item_name)
            .map(|(el, _)| T::from(el))
    }
}

impl<E: HasChildrenMut, T> List<E, T> {
    pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = T>
    where
        T: From<&'a mut Element>,
    {
        self.element
            .elements_by_name_mut(self.item_name)
            .map(|(el, _)| T::from(el))
    }

    // TODO: insert, remove, push, pop
}

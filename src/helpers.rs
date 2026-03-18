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
    pub fn len<E1>(&self) -> usize
    where
        T: FromElement<E1>,
    {
        self.element.elements_by_name(self.item_name).count()
    }

    pub fn is_empty<E1>(&self) -> bool
    where
        T: FromElement<E1>,
    {
        self.len() == 0
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = T>
    where
        T: FromElement<&'a Element>,
    {
        self.element
            .elements_by_name(self.item_name)
            .map(|(el, _)| T::from_element(el))
    }
}

impl<E: HasChildrenMut, T> List<E, T> {
    pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = T>
    where
        T: FromElement<&'a mut Element>,
    {
        self.element
            .elements_by_name_mut(self.item_name)
            .map(|(el, _)| T::from_element(el))
    }

    // TODO: insert, remove, push, pop
}

pub trait FromElement<E> {
    /// Constructs from either `&Element` or `&mut Element`.
    fn from_element(element: E) -> Self;
}

use crate::domtree::{Element, HasChildren, HasChildrenMut};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct List<E, T> {
    pub(crate) element: E,
    _marker: PhantomData<T>,
}

impl<E, T> List<E, T> {
    pub fn new(element: E) -> Self {
        Self {
            element,
            _marker: PhantomData,
        }
    }
}

impl<E: HasChildren, T> List<E, T> {
    pub fn len<E1>(&self) -> usize
    where
        T: ListItem<E1>,
    {
        self.element.elements_by_name(T::NAME).count()
    }

    pub fn is_empty<E1>(&self) -> bool
    where
        T: ListItem<E1>,
    {
        self.len() == 0
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = T>
    where
        T: ListItem<&'a Element>,
    {
        self.element
            .elements_by_name(T::NAME)
            .map(|(el, _)| T::from_element(el))
    }
}

impl<E: HasChildrenMut, T> List<E, T> {
    pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = T>
    where
        T: ListItem<&'a mut Element>,
    {
        self.element
            .elements_by_name_mut(T::NAME)
            .map(|(el, _)| T::from_element(el))
    }

    // TODO: insert, remove, push, pop
}

pub trait ListItem<E> {
    const NAME: &'static str;

    fn from_element(element: E) -> Self;
}

use std::{fmt::Debug, marker::PhantomData, ptr::NonNull};

use crate::{
    Ptr,
    core::{BSTree, Node},
};

pub struct Iter<'a, T> {
    current: Ptr<Node<T>>,
    stack: Vec<NonNull<Node<T>>>,
    _market: PhantomData<&'a T>,
}

impl<T: PartialOrd + Debug + Clone> BSTree<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        let stack = Vec::<NonNull<Node<T>>>::new();
        Iter {
            current: self.root,
            stack,
            _market: PhantomData,
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            while let Some(node) = self.current {
                self.stack.push(node);
                self.current = (*node.as_ptr()).left;
            }

            let node = self.stack.pop()?;

            self.current = (*node.as_ptr()).right;

            Some(&(*node.as_ptr()).data)
        }
    }
}

impl<'a, T> IntoIterator for &'a BSTree<T>
where
    T: PartialOrd + Debug + Clone,
{
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

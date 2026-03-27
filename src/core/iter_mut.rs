use std::{fmt::Debug, marker::PhantomData, ptr::NonNull};

use crate::{
    Ptr,
    core::{BSTree, Node},
};

pub struct IterMut<'a, T> {
    current: Ptr<Node<T>>,
    stack: Vec<NonNull<Node<T>>>,
    _market: PhantomData<&'a mut T>,
}

impl<T: PartialOrd + Debug + Clone> BSTree<T> {
    pub fn iter_mut(&mut self) -> IterMut<T> {
        let stack = Vec::<NonNull<Node<T>>>::new();
        IterMut {
            current: self.root,
            stack,
            _market: PhantomData,
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            while let Some(node) = self.current {
                self.stack.push(node);
                self.current = (*node.as_ptr()).left;
            }

            let node = self.stack.pop()?;

            self.current = (*node.as_ptr()).right;

            Some(&mut (*node.as_ptr()).data)
        }
    }
}

impl<'a, T> IntoIterator for &'a mut BSTree<T>
where
    T: PartialOrd + Debug + Clone,
{
    type IntoIter = IterMut<'a, T>;
    type Item = &'a mut T;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

use std::fmt::Debug;

use crate::{
    Ptr,
    core::{BSTree, Node},
};

pub(crate) enum Child<T> {
    Empty,
    Left(Ptr<Node<T>>),
    Right(Ptr<Node<T>>),
    //TODO: Both((Ptr<Node<T>>, Ptr<Node<T>>)),
}

impl<T> BSTree<T>
where
    T: PartialOrd + Debug + Clone,
{
    pub(crate) fn selectable(node: Ptr<Node<T>>) -> Child<T> {
        match node {
            None => Child::Empty,
            Some(node) => unsafe {
                //if (*node.as_ptr()).left.is_some() && (*node.as_ptr()).right.is_some() {
                //Child::Both(((*node.as_ptr()).left, (*node.as_ptr()).right))
                //} else
                if (*node.as_ptr()).left.is_some() {
                    Child::Left((*node.as_ptr()).left)
                } else if (*node.as_ptr()).right.is_some() {
                    Child::Right((*node.as_ptr()).right)
                } else {
                    Child::Empty
                }
            },
        }
    }
}

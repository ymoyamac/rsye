use std::{fmt::Debug, ptr::NonNull};

use crate::core::{BSTree, Node};

impl<T: PartialOrd + Debug> BSTree<T> {
    fn rotate_right(&mut self, node: NonNull<Node<T>>) {
        //      z                y
        //     /                / \
        //    y        →       x   z
        //   /
        //  x
        unsafe {
            let z_parent = (*node.as_ptr()).parent;
            let y = (*node.as_ptr()).left.unwrap();
            let y_right = (*y.as_ptr()).right;
            (*y.as_ptr()).right = Some(node);
            (*node.as_ptr()).left = y_right;
            if let Some(yr) = y_right {
                (*yr.as_ptr()).parent = Some(node);
            }
            (*y.as_ptr()).parent = z_parent;
            (*node.as_ptr()).parent = Some(y);
            match z_parent {
                Some(parent) => {
                    if (*parent.as_ptr()).left == Some(node) {
                        (*parent.as_ptr()).left = Some(y);
                    } else {
                        (*parent.as_ptr()).right = Some(y);
                    }
                }
                None => {
                    *self.root_ptr_mut() = Some(y);
                }
            }

            Self::update_height(node);
            Self::update_height(y);
        }
    }
}

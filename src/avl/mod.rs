use std::{fmt::Debug, ptr::NonNull};

use crate::core::{BSTree, Node};

impl<T: PartialOrd + Debug> BSTree<T> {
    pub(crate) fn update_height(node: NonNull<Node<T>>) {
        unsafe {
            let left_height = match (*node.as_ptr()).left {
                Some(left) => (*left.as_ptr()).height,
                None => 0,
            };
            let right_height = match (*node.as_ptr()).right {
                Some(right) => (*right.as_ptr()).height,
                None => 0,
            };
            (*node.as_ptr()).height = 1 + left_height.max(right_height);
        }
    }

    pub(crate) fn walk_up(&mut self, node: NonNull<Node<T>>) {
        let mut current = Some(node);
        unsafe {
            while let Some(n) = current {
                Self::update_height(n);
                self.rebalance(n);
                current = (*n.as_ptr()).parent;
            }
        }
    }

    pub(crate) fn balance_factor(node: NonNull<Node<T>>) -> i32 {
        unsafe {
            let left_height = match (*node.as_ptr()).left {
                Some(left) => (*left.as_ptr()).height as i32,
                None => 0,
            };

            let right_height = match (*node.as_ptr()).right {
                Some(right) => (*right.as_ptr()).height as i32,
                None => 0,
            };
            left_height - right_height
        }
    }

    pub(crate) fn rebalance(&mut self, node: NonNull<Node<T>>) {
        let balance = Self::balance_factor(node);
        unsafe {
            if balance == 2 {
                if Self::balance_factor((*node.as_ptr()).left.unwrap()) >= 0 {
                    self.rotate_right(node);
                } else {
                    self.rotate_left_right(node);
                }
            } else if balance == -2 {
                if Self::balance_factor((*node.as_ptr()).right.unwrap()) <= 0 {
                    self.rotate_left(node);
                } else {
                    self.rotate_right_left(node);
                }
            }
        }
    }

    pub(crate) fn rotate_right(&mut self, node: NonNull<Node<T>>) {
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
                None => *self.root_mut().unwrap() = y,
            }

            Self::update_height(node);
            Self::update_height(y);
        }
    }

    pub(crate) fn rotate_left(&mut self, node: NonNull<Node<T>>) {
        //      x                y
        //       \              / \
        //        y    →       x   z
        //        \
        //        z
        unsafe {
            let x_parent = (*node.as_ptr()).parent;
            let y = (*node.as_ptr()).right.unwrap();
            let y_left = (*y.as_ptr()).left;
            (*y.as_ptr()).left = Some(node);
            (*node.as_ptr()).right = y_left;
            if let Some(yl) = y_left {
                (*yl.as_ptr()).parent = Some(node);
            }
            (*y.as_ptr()).parent = x_parent;
            (*node.as_ptr()).parent = Some(y);
            match x_parent {
                Some(parent) => {
                    if (*parent.as_ptr()).left == Some(node) {
                        (*parent.as_ptr()).left = Some(y);
                    } else {
                        (*parent.as_ptr()).right = Some(y);
                    }
                }
                None => *self.root_mut().unwrap() = y,
            }
            Self::update_height(node);
            Self::update_height(y);
        }
    }

    pub(crate) fn rotate_left_right(&mut self, node: NonNull<Node<T>>) {
        unsafe {
            let w = (*node.as_ptr()).left.unwrap();
            self.rotate_left(w);
            self.rotate_right(node);
        }
    }

    pub(crate) fn rotate_right_left(&mut self, node: NonNull<Node<T>>) {
        unsafe {
            let z = (*node.as_ptr()).right.unwrap();
            self.rotate_right(z);
            self.rotate_left(node);
        }
    }
}

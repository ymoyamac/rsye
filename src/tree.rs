use std::ptr::NonNull;

use crate::{Ptr, node::Node};

pub struct Tree<T>
where
    T: PartialOrd,
{
    pub root: Ptr<Node<T>>,
    pub levels: u32,
}

impl<T: PartialOrd> Default for Tree<T> {
    fn default() -> Self {
        Self {
            root: None,
            levels: 0,
        }
    }
}

impl<T: PartialOrd> Tree<T> {
    pub fn new() -> Self {
        Self {
            root: None,
            levels: 0,
        }
    }

    fn insert(&mut self, data: T) {
        unsafe {
            if let Some(mut root) = self.root {
                loop {
                    if data < (*root.as_ptr()).data {
                        if let Some(left) = (*root.as_ptr()).left {
                            root = left;
                        } else {
                            let node = Node::new_ptr(data);
                            (*node.as_ptr()).parent = Some(root);
                            (*root.as_ptr()).left = Some(node);
                            self.levels += 1;
                            break;
                        }
                    } else if data > (*root.as_ptr()).data {
                        if let Some(right) = (*root.as_ptr()).right {
                            root = right;
                        } else {
                            let node = Node::new_ptr(data);
                            (*node.as_ptr()).parent = Some(root);
                            (*root.as_ptr()).right = Some(node);
                            self.levels += 1;
                            break;
                        }
                    } else {
                        break;
                    }
                }
            } else {
                let node = Node::new_ptr(data);
                self.root = Some(node);
                self.levels += 1;
            }
        }
    }
}

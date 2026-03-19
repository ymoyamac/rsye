use crate::{Ptr, node::Node};
use std::fmt::Debug;
use std::ptr::NonNull;

#[derive(Debug)]
pub struct BSTree<T>
where
    T: PartialOrd + Debug,
{
    pub root: Ptr<Node<T>>,
    pub height: u32,
}

impl<T: PartialOrd + Debug> Default for BSTree<T> {
    fn default() -> Self {
        Self {
            root: None,
            height: 0,
        }
    }
}

impl<T: PartialOrd + Debug> BSTree<T> {
    pub fn new() -> Self {
        Self {
            root: None,
            height: 0,
        }
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn insert(&mut self, data: T) {
        unsafe {
            let node = Node::new_ptr(data);

            let Some(mut current) = self.root else {
                self.root = Some(node);
                self.height += 1;
                return;
            };

            loop {
                if (*node.as_ptr()).data < (*current.as_ptr()).data {
                    match (*current.as_ptr()).left {
                        Some(left) => current = left,
                        None => {
                            (*node.as_ptr()).parent = Some(current);
                            (*current.as_ptr()).left = Some(node);
                            self.height += 1;
                            return;
                        }
                    }
                } else if (*node.as_ptr()).data > (*current.as_ptr()).data {
                    match (*current.as_ptr()).right {
                        Some(right) => current = right,
                        None => {
                            (*node.as_ptr()).parent = Some(current);
                            (*current.as_ptr()).right = Some(node);
                            self.height += 1;
                            return;
                        }
                    }
                } else {
                    drop(Box::from_raw(node.as_ptr()));
                    return;
                }
            }
        }
    }

    //Breadth-First Search
    pub fn bfs(&mut self, to_find: T) -> Option<&T> {
        let root = self.root?;
        use std::collections::VecDeque;
        let mut queue = VecDeque::<NonNull<Node<T>>>::new();
        unsafe {
            queue.push_back(root);
            while let Some(current) = queue.pop_front() {
                if (*current.as_ptr()).data == to_find {
                    return Some(&(*current.as_ptr()).data);
                }
                if let Some(left) = (*current.as_ptr()).left {
                    queue.push_back(left);
                }
                if let Some(right) = (*current.as_ptr()).right {
                    queue.push_back(right);
                }
            }
        }
        None
    }
}

use crate::{Ptr, node::Node};

pub struct Tree<T>
where
    T: PartialOrd,
{
    pub root: Ptr<Node<T>>,
    pub height: u32,
}

impl<T: PartialOrd> Default for Tree<T> {
    fn default() -> Self {
        Self {
            root: None,
            height: 0,
        }
    }
}

impl<T: PartialOrd> Tree<T> {
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
}

use crate::{Ptr, core::Node};
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

    pub fn root(&self) -> Option<&T> {
        unsafe { Some(&(*self.root.unwrap().as_ptr()).data) }
    }

    pub fn root_mut(&self) -> Option<&mut T> {
        unsafe { Some(&mut (*self.root.unwrap().as_ptr()).data) }
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
    pub fn bfs(&self, to_find: T) -> Option<&T> {
        use std::collections::VecDeque;
        let root = self.root?;
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

    //Depth-First Search
    pub fn dfs(&self, to_find: T) -> Option<&T> {
        let mut stack = Vec::<NonNull<Node<T>>>::new();
        let root = self.root?;
        unsafe {
            stack.push(root);
            while let Some(current) = stack.pop() {
                if (*current.as_ptr()).data == to_find {
                    return Some(&(*current.as_ptr()).data);
                }
                if let Some(right) = (*current.as_ptr()).right {
                    stack.push(right);
                }
                if let Some(left) = (*current.as_ptr()).left {
                    stack.push(left);
                }
            }
        }
        None
    }

    //inorder
    pub fn to_vec(self) -> Vec<T> {
        let mut values = Vec::<T>::new();
        let mut stack = Vec::<NonNull<Node<T>>>::new();
        unsafe {
            let mut current = self.root;
            while current.is_some() || !stack.is_empty() {
                if let Some(node) = current {
                    stack.push(node);
                    current = (*node.as_ptr()).left;
                } else {
                    let last = stack.pop().unwrap();
                    values.push(std::ptr::read(&(*last.as_ptr()).data));
                    current = (*last.as_ptr()).right;
                }
            }
        }
        values
    }
}

impl<T: PartialOrd + Debug, const N: usize> From<[T; N]> for BSTree<T> {
    fn from(values: [T; N]) -> Self {
        let mut tree = BSTree::<T>::new();

        for v in values {
            tree.insert(v);
        }
        tree
    }
}

impl<T: PartialOrd + Debug> Drop for BSTree<T> {
    fn drop(&mut self) {
        let mut path = Vec::<NonNull<Node<T>>>::new();
        let mut nodes = Vec::<NonNull<Node<T>>>::new();
        unsafe {
            if let Some(root) = self.root {
                path.push(root);
                while let Some(current) = path.pop() {
                    nodes.push(current);
                    if let Some(left) = (*current.as_ptr()).left {
                        path.push(left);
                    }
                    if let Some(right) = (*current.as_ptr()).right {
                        path.push(right);
                    }
                }
            }
            while let Some(current) = nodes.pop() {
                drop(Box::from_raw(current.as_ptr()));
            }
        }
    }
}

use crate::{Ptr, core::Node};
use std::fmt::Debug;
use std::ptr::NonNull;

#[derive(Debug)]
pub struct BSTree<T>
where
    T: PartialOrd + Debug,
{
    root: Ptr<Node<T>>,
    nodes: u32,
}

impl<T: PartialOrd + Debug> Default for BSTree<T> {
    fn default() -> Self {
        Self {
            root: None,
            nodes: 0,
        }
    }
}

impl<T: PartialOrd + Debug> BSTree<T> {
    pub fn new() -> Self {
        Self {
            root: None,
            nodes: 0,
        }
    }

    pub fn data_root(&self) -> Option<&T> {
        self.root.map(|root| unsafe { &(*root.as_ptr()).data })
    }

    pub fn data_root_mut(&self) -> Option<&mut T> {
        self.root.map(|root| unsafe { &mut (*root.as_ptr()).data })
    }

    pub fn root(&self) -> Option<&NonNull<Node<T>>> {
        self.root.as_ref()
    }

    pub fn root_mut(&mut self) -> Option<&mut NonNull<Node<T>>> {
        self.root.as_mut()
    }

    pub fn left(&self) -> Option<&NonNull<Node<T>>> {
        self.root
            .as_ref()
            .map(|root| unsafe { (*root.as_ptr()).left.as_ref() })?
    }

    pub fn right(&self) -> Option<&NonNull<Node<T>>> {
        self.root
            .as_ref()
            .map(|root| unsafe { (*root.as_ptr()).right.as_ref() })?
    }

    pub fn nodes(&self) -> u32 {
        self.nodes
    }

    pub fn height(&self) -> u32 {
        match self.root {
            Some(root) => unsafe { (*root.as_ptr()).height },
            None => 0,
        }
    }

    pub fn len(&self) -> u32 {
        self.nodes
    }

    pub fn is_empty(&self) -> bool {
        self.nodes == 0
    }

    pub fn insert(&mut self, data: T) {
        unsafe {
            let node = Node::new_ptr(data);

            let Some(mut current) = self.root else {
                self.root = Some(node);
                self.nodes += 1;
                Self::update_height(node);
                return;
            };

            loop {
                if (*node.as_ptr()).data < (*current.as_ptr()).data {
                    match (*current.as_ptr()).left {
                        Some(left) => current = left,
                        None => {
                            (*node.as_ptr()).parent = Some(current);
                            (*current.as_ptr()).left = Some(node);
                            self.nodes += 1;
                            Self::walk_up(self, node);
                            return;
                        }
                    }
                } else if (*node.as_ptr()).data > (*current.as_ptr()).data {
                    match (*current.as_ptr()).right {
                        Some(right) => current = right,
                        None => {
                            (*node.as_ptr()).parent = Some(current);
                            (*current.as_ptr()).right = Some(node);
                            self.nodes += 1;
                            Self::walk_up(self, node);
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

    pub fn search(&self, to_find: T) -> Option<&T> {
        let mut root = self.root;
        unsafe {
            while let Some(current) = root {
                if to_find < (*current.as_ptr()).data {
                    root = (*current.as_ptr()).left;
                } else if to_find > (*current.as_ptr()).data {
                    root = (*current.as_ptr()).right;
                } else {
                    return Some(&(*current.as_ptr()).data);
                }
            }
        }
        None
    }

    //inorder
    pub fn as_vec(self) -> Vec<T> {
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

    pub fn contains(&self, data: T) -> bool {
        self.search(data).is_some()
    }

    pub fn min(&self) -> Option<&T> {
        let mut iter = self.root;
        unsafe {
            while let Some(current) = iter {
                if (*current.as_ptr()).left.is_some() {
                    iter = (*current.as_ptr()).left;
                } else {
                    break;
                }
            }
            iter.as_ref().map(|node| &(*node.as_ptr()).data)
        }
    }

    pub fn max(&self) -> Option<&T> {
        let mut iter = self.root;
        unsafe {
            while let Some(current) = iter {
                if (*current.as_ptr()).right.is_some() {
                    iter = (*current.as_ptr()).right;
                } else {
                    break;
                }
            }
            iter.as_ref().map(|node| &(*node.as_ptr()).data)
        }
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

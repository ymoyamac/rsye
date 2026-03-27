use std::{marker::PhantomData, ptr::NonNull};

use crate::Ptr;

pub struct Node<T> {
    pub data: T,
    pub height: u32,
    pub parent: Ptr<Self>,
    pub left: Ptr<Self>,
    pub right: Ptr<Self>,
    _marker: PhantomData<T>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            height: 0,
            parent: None,
            left: None,
            right: None,
            _marker: PhantomData,
        }
    }

    pub fn new_ptr(data: T) -> NonNull<Self> {
        unsafe {
            NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                data,
                height: 0,
                parent: None,
                left: None,
                right: None,
                _marker: PhantomData,
            })))
        }
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

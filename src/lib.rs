use std::ptr::NonNull;

pub mod node;
pub mod tree;

pub(crate) type Ptr<T> = Option<NonNull<T>>;


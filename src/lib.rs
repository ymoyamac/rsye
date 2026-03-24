use std::ptr::NonNull;
pub mod avl;
pub mod core;

pub(crate) type Ptr<T> = Option<NonNull<T>>;

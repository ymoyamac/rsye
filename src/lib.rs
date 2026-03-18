use std::ptr::NonNull;

pub mod bstree;
pub mod node;

pub use bstree::BSTree;
pub use node::Node;

pub(crate) type Ptr<T> = Option<NonNull<T>>;

use std::ptr::NonNull;

pub mod node;
pub mod tree;

pub use node::Node;
pub use tree::Tree;

pub(crate) type Ptr<T> = Option<NonNull<T>>;

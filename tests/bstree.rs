use rsye::core::BSTree;

#[test]
fn new_tree() {
    let tree = BSTree::<i32>::new();
    assert_eq!(0, tree.height());
}

#[test]
fn insert_from_slice() {
    let values = [1, 2, 3, 4];
    let tree = BSTree::<i32>::from(values);
    assert_eq!(tree.root().unwrap(), &values[0]);
}

#[test]
fn new_tree_is_empty() {
    let tree: BSTree<i32> = BSTree::new();
    assert!(tree.root.is_none());
    assert_eq!(tree.height(), 0);
}

#[test]
fn default_tree_is_empty() {
    let tree: BSTree<i32> = BSTree::default();
    assert!(tree.root.is_none());
    assert_eq!(tree.height(), 0);
}

#[test]
fn insert_single_node_becomes_root() {
    let tree = BSTree::from([1]);
    assert_eq!(tree.root(), Some(&1));
}

#[test]
fn insert_smaller_goes_left() {
    let tree = BSTree::from([5, 3]);
    unsafe {
        let left = (*tree.root.unwrap().as_ptr()).left.unwrap();
        assert_eq!((*left.as_ptr()).data, 3);
    }
}

#[test]
fn insert_larger_goes_right() {
    let tree = BSTree::from([5, 7]);
    unsafe {
        let right = (*tree.root.unwrap().as_ptr()).right.unwrap();
        assert_eq!((*right.as_ptr()).data, 7);
    }
}

#[test]
fn insert_duplicate_is_ignored() {
    let tree = BSTree::from([5, 5, 5]);
    assert_eq!(tree.height(), 1);
}

#[test]
fn insert_updates_parent_pointer() {
    let tree = BSTree::from([5, 3]);
    unsafe {
        let left = (*tree.root.unwrap().as_ptr()).left.unwrap();
        let parent = (*left.as_ptr()).parent.unwrap();
        assert_eq!((*parent.as_ptr()).data, 5);
    }
}

#[test]
fn insert_updates_height() {
    let tree = BSTree::from([5, 3, 7, 1, 4]);
    assert_eq!(tree.height(), 5);
}

#[test]
fn bfs_finds_root() {
    let tree = BSTree::from([5, 3, 7]);
    assert_eq!(tree.bfs(5), Some(&5));
}

#[test]
fn bfs_finds_left_leaf() {
    let tree = BSTree::from([5, 3, 7]);
    assert_eq!(tree.bfs(3), Some(&3));
}

#[test]
fn bfs_finds_right_leaf() {
    let tree = BSTree::from([5, 3, 7]);
    assert_eq!(tree.bfs(7), Some(&7));
}

#[test]
fn bfs_returns_none_when_not_found() {
    let tree = BSTree::from([5, 3, 7]);
    assert_eq!(tree.bfs(99), None);
}

#[test]
fn bfs_on_empty_tree_returns_none() {
    let tree: BSTree<i32> = BSTree::new();
    assert_eq!(tree.bfs(1), None);
}

#[test]
fn bfs_finds_deep_node() {
    let tree = BSTree::from([5, 3, 7, 1, 4, 6, 8]);
    assert_eq!(tree.bfs(1), Some(&1));
}

#[test]
fn dfs_finds_root() {
    let tree = BSTree::from([5, 3, 7]);
    assert_eq!(tree.dfs(5), Some(&5));
}

#[test]
fn dfs_finds_left_leaf() {
    let tree = BSTree::from([5, 3, 7]);
    assert_eq!(tree.dfs(3), Some(&3));
}

#[test]
fn dfs_finds_right_leaf() {
    let tree = BSTree::from([5, 3, 7]);
    assert_eq!(tree.dfs(7), Some(&7));
}

#[test]
fn dfs_finds_deep_node() {
    let tree = BSTree::from([5, 3, 7, 1, 4]);
    assert_eq!(tree.dfs(1), Some(&1));
}

#[test]
fn dfs_returns_none_when_not_found() {
    let tree = BSTree::from([5, 3, 7]);
    assert_eq!(tree.dfs(99), None);
}

#[test]
fn dfs_on_empty_tree_returns_none() {
    let tree: BSTree<i32> = BSTree::new();
    assert_eq!(tree.dfs(1), None);
}

#[test]
fn to_vec_returns_sorted_values() {
    let tree = BSTree::from([5, 3, 7, 1, 4]);
    assert_eq!(tree.to_vec(), vec![1, 3, 4, 5, 7]);
}

#[test]
fn to_vec_single_node() {
    let tree = BSTree::from([42]);
    assert_eq!(tree.to_vec(), vec![42]);
}

#[test]
fn to_vec_empty_tree() {
    let tree: BSTree<i32> = BSTree::new();
    assert_eq!(tree.to_vec(), vec![]);
}

#[test]
fn to_vec_already_sorted_input() {
    let tree = BSTree::from([1, 2, 3, 4, 5]);
    assert_eq!(tree.to_vec(), vec![1, 2, 3, 4, 5]);
}

#[test]
fn to_vec_reverse_sorted_input() {
    let tree = BSTree::from([5, 4, 3, 2, 1]);
    assert_eq!(tree.to_vec(), vec![1, 2, 3, 4, 5]);
}

#[test]
fn to_vec_with_negative_values() {
    let tree = BSTree::from([0, -1, 5, 3, 4]);
    assert_eq!(tree.to_vec(), vec![-1, 0, 3, 4, 5]);
}

#[test]
fn from_array_builds_correct_tree() {
    let tree = BSTree::from([5, 3, 7]);
    assert_eq!(tree.root(), Some(&5));
    assert_eq!(tree.height(), 3);
}

#[test]
fn from_single_element_array() {
    let tree = BSTree::from([42]);
    assert_eq!(tree.root(), Some(&42));
    assert_eq!(tree.height(), 1);
}

#[test]
fn drop_does_not_leak_memory() {
    let tree = BSTree::from([5, 3, 7, 1, 4, 6, 8]);
    drop(tree);
}

#[test]
fn drop_empty_tree_does_not_panic() {
    let tree: BSTree<i32> = BSTree::new();
    drop(tree);
}

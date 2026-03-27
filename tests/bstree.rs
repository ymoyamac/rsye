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
    assert_eq!(tree.data_root(), Some(&2));
}

#[test]
fn new_tree_is_empty() {
    let tree: BSTree<i32> = BSTree::new();
    assert!(tree.root().is_none());
    assert_eq!(tree.height(), 0);
}

#[test]
fn default_tree_is_empty() {
    let tree: BSTree<i32> = BSTree::default();
    assert!(tree.root().is_none());
    assert_eq!(tree.height(), 0);
}

#[test]
fn insert_single_node_becomes_root() {
    let tree = BSTree::from([1]);
    assert_eq!(tree.data_root(), Some(&1));
}

#[test]
fn insert_smaller_goes_left() {
    let tree = BSTree::from([5, 3]);
    let left = tree.root_left().unwrap();
    unsafe {
        assert_eq!((*left.as_ptr()).data, 3);
    }
}

#[test]
fn insert_larger_goes_right() {
    let tree = BSTree::from([5, 7]);
    let right = tree.root_right().unwrap();
    unsafe {
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
        let left = tree.root_left().unwrap();
        let parent = (*left.as_ptr()).parent.unwrap();
        assert_eq!((*parent.as_ptr()).data, 5);
    }
}

#[test]
fn insert_updates_height() {
    let tree = BSTree::from([5, 3, 7, 1, 4]);
    assert_eq!(tree.height(), 3);
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
    assert_eq!(tree.as_vec(), vec![1, 3, 4, 5, 7]);
}

#[test]
fn to_vec_single_node() {
    let tree = BSTree::from([42]);
    assert_eq!(tree.as_vec(), vec![42]);
}

#[test]
fn to_vec_empty_tree() {
    let tree: BSTree<i32> = BSTree::new();
    assert_eq!(tree.as_vec(), vec![]);
}

#[test]
fn to_vec_already_sorted_input() {
    let tree = BSTree::from([1, 2, 3, 4, 5]);
    assert_eq!(tree.as_vec(), vec![1, 2, 3, 4, 5]);
}

#[test]
fn to_vec_reverse_sorted_input() {
    let tree = BSTree::from([5, 4, 3, 2, 1]);
    assert_eq!(tree.as_vec(), vec![1, 2, 3, 4, 5]);
}

#[test]
fn to_vec_with_negative_values() {
    let tree = BSTree::from([0, -1, 5, 3, 4]);
    assert_eq!(tree.as_vec(), vec![-1, 0, 3, 4, 5]);
}

#[test]
fn from_array_builds_correct_tree() {
    let tree = BSTree::from([5, 3, 7]);
    assert_eq!(tree.data_root(), Some(&5));
    assert_eq!(tree.height(), 2);
}

#[test]
fn from_single_element_array() {
    let tree = BSTree::from([42]);
    assert_eq!(tree.data_root(), Some(&42));
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

#[test]
fn search_finds_root() {
    let tree = BSTree::from([5, 3, 7]);
    assert_eq!(tree.search(5), Some(&5));
}

#[test]
fn search_finds_left_node() {
    let tree = BSTree::from([5, 3, 7]);
    assert_eq!(tree.search(3), Some(&3));
}

#[test]
fn search_finds_right_node() {
    let tree = BSTree::from([5, 3, 7]);
    assert_eq!(tree.search(7), Some(&7));
}

#[test]
fn search_returns_none_when_not_found() {
    let tree = BSTree::from([5, 3, 7]);
    assert_eq!(tree.search(99), None);
}

#[test]
fn search_on_empty_tree_returns_none() {
    let tree: BSTree<i32> = BSTree::new();
    assert_eq!(tree.search(1), None);
}

#[test]
fn search_finds_deep_node() {
    let tree = BSTree::from([10, 5, 15, 3, 7, 12, 20]);
    assert_eq!(tree.search(3), Some(&3));
}

#[test]
fn contains() {
    let tree = BSTree::from([10, 5, 15, 3, 7, 12, 20]);
    assert_eq!(tree.len(), 7);
    assert!(tree.contains(12));
    assert!(!tree.is_empty());
}

#[test]
fn min() {
    let tree = BSTree::from([10, 5, 15, 3, 7, 12, 20]);
    assert_eq!(tree.min(), Some(&3));
}

#[test]
fn max() {
    let tree = BSTree::from([10, 5, 15, 3, 7, 99, 12, 20]);
    assert_eq!(tree.max(), Some(&99));
}

#[test]
fn range_returns_subtree_with_values_in_range() {
    let tree = BSTree::from([5, 3, 8, 1, 4, 9]);
    let sub = tree.range(3, 7);
    assert_eq!(sub.as_vec(), vec![3, 4, 5]);
}

#[test]
fn range_full_range_returns_all_values() {
    let tree = BSTree::from([5, 3, 7]);
    let sub = tree.range(3, 7);
    assert_eq!(sub.as_vec(), vec![3, 5, 7]);
}

#[test]
fn range_empty_result_when_no_values_in_range() {
    let tree = BSTree::from([5, 3, 7]);
    let sub = tree.range(10, 20);
    assert!(sub.is_empty());
}

#[test]
fn range_single_value() {
    let tree = BSTree::from([5, 3, 7]);
    let sub = tree.range(5, 5);
    assert_eq!(sub.as_vec(), vec![5]);
}

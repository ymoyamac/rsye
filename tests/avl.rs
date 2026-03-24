use rsye::core::BSTree;

#[test]
fn insert_rebalances_left_heavy() {
    let tree = BSTree::from([5, 3, 1]);
    assert_eq!(tree.data_root(), Some(&3));
    assert_eq!(tree.as_vec(), vec![1, 3, 5]);
}

#[test]
fn insert_rebalances_right_heavy() {
    let tree = BSTree::from([1, 3, 5]);
    assert_eq!(tree.data_root(), Some(&3));
    assert_eq!(tree.as_vec(), vec![1, 3, 5]);
}

#[test]
fn insert_rebalances_left_right() {
    let tree = BSTree::from([5, 1, 3]);
    assert_eq!(tree.data_root(), Some(&3));
    assert_eq!(tree.as_vec(), vec![1, 3, 5]);
}

#[test]
fn insert_rebalances_right_left() {
    let tree = BSTree::from([1, 5, 3]);
    assert_eq!(tree.data_root(), Some(&3));
    assert_eq!(tree.as_vec(), vec![1, 3, 5]);
}

#[test]
fn insert_larger_tree_remains_balanced() {
    let tree = BSTree::from([10, 5, 15, 3, 7, 12, 20]);
    assert_eq!(tree.height(), 3);
    assert_eq!(tree.as_vec(), vec![3, 5, 7, 10, 12, 15, 20]);
}

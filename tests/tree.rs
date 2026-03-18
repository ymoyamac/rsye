use rsye::Tree;

#[test]
fn new_tree() {
    let tree = Tree::<i32>::new();
    assert_eq!(0, tree.height());
}

#[test]
fn insert_tree() {
    let mut tree = Tree::<i32>::new();
    tree.insert(1);
    tree.insert(3);
    tree.insert(5);
    assert_eq!(1, tree.height());
}

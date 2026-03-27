use rsye::core::BSTree;

#[test]
fn iter() {
    let tree = BSTree::from([1, 2, 3]);
    for v in tree.iter() {
        dbg!(v);
    }
    let mut iter = tree.iter();
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), None);
}

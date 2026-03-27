use rsye::core::BSTree;

#[test]
fn iter_mut() {
    let mut tree = BSTree::from([1, 2, 3, 4, 5, 6]);
    let mut iter = tree.iter_mut();
    assert_eq!(iter.next(), Some(&mut 1));
    assert_eq!(iter.next(), Some(&mut 2));
    assert_eq!(iter.next(), Some(&mut 3));
    assert_eq!(iter.next(), Some(&mut 4));
    assert_eq!(iter.next(), Some(&mut 5));
    assert_eq!(iter.next(), Some(&mut 6));
    assert_eq!(iter.next(), None);
}

#[test]
fn iter_mut_map() {
    let mut tree = BSTree::from([1, 2, 3, 4, 5, 6]);
    for v in tree.iter_mut() {
        *v *= 2;
    }

    let mut iter = tree.iter_mut();
    assert_eq!(iter.next(), Some(&mut 2));
    assert_eq!(iter.next(), Some(&mut 4));
    assert_eq!(iter.next(), Some(&mut 6));
    assert_eq!(iter.next(), Some(&mut 8));
    assert_eq!(iter.next(), Some(&mut 10));
    assert_eq!(iter.next(), Some(&mut 12));
    assert_eq!(iter.next(), None);
}

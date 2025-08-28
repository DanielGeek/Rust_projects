use generic_trait::*;

#[test]
fn test_tuple_iter() {
    let mut iter: TupleIter<u32> = TupleIter {
        tuple: (10, 20, 30),
        next: 0,
    };

    assert_eq!(iter.next(), Some(10).as_ref());
    assert_eq!(iter.next(), Some(20).as_ref());
    assert_eq!(iter.next(), Some(30).as_ref());
    assert_eq!(iter.next(), None);
}

#[test]
fn test_vec_iter() {
    let mut iter: VecIter<u32> = VecIter {
        vec: vec![10, 20, 30],
        next: 0,
    };

    assert_eq!(iter.next(), Some(10).as_ref());
    assert_eq!(iter.next(), Some(20).as_ref());
    assert_eq!(iter.next(), Some(30).as_ref());
    assert_eq!(iter.next(), None);
}

use tuple::*;

#[test]
fn test_first() {
    assert_eq!(first((true, 1, 'a')), true);
    assert_eq!(first((false, 0, 'b')), false);
}

#[test]
fn test_last() {
    assert_eq!(last((true, 1, 'a')), 'a');
    assert_eq!(last((false, 0, 'b')), 'b');
}

#[test]
fn test_swap() {
    assert_eq!(swap((1, 2)), (2, 1));
    assert_eq!(swap((2, 1)), (1, 2));
    assert_eq!(swap((1, 1)), (1, 1));
}

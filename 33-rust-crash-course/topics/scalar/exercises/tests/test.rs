use scalar::*;

#[test]
fn test_eq() {
    assert!(eq('a', 'a'));
    assert!(eq('b', 'b'));
    assert!(!eq('a', 'b'));
    assert!(!eq('b', 'a'));
}

#[test]
fn test_add() {
    assert_eq!(add(1.0, 2.0, 3.0), 6.0);
    assert_eq!(add(1.0, 2.0, -3.0), 0.0);
}

#[test]
fn test_cast() {
    assert_eq!(cast(1u8, 2i8, 3.0), 6.0);
    assert_eq!(cast(1u8, 2i8, -3.0), 0.0);
}

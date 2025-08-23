use if_else::*;

#[test]
fn test_min() {
    assert_eq!(min(1, 2), 1);
    assert_eq!(min(2, 1), 1);
    assert_eq!(min(2, 2), 2);
}

#[test]
fn test_max() {
    assert_eq!(max(1, 2), 2);
    assert_eq!(max(2, 1), 2);
    assert_eq!(max(2, 2), 2);
}

#[test]
fn test_sign() {
    assert_eq!(sign(10), 1);
    assert_eq!(sign(0), 1);
    assert_eq!(sign(-10), -1);
}

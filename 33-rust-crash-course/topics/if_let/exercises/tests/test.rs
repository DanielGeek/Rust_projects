use if_let::*;

#[test]
fn test_unwrap_or_default() {
    assert_eq!(unwrap_or_default(Some(1), 0), 1);
    assert_eq!(unwrap_or_default(Some(2), 0), 2);
    assert_eq!(unwrap_or_default(None, 0), 0);
}

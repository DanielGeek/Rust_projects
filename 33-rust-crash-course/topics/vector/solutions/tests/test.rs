use vector::*;

#[test]
fn test_init() {
    let v = init(1, 2, 3);
    assert_eq!(v.len(), 3);
    assert_eq!(v.get(0), Some(1).as_ref());
    assert_eq!(v.get(1), Some(2).as_ref());
    assert_eq!(v.get(2), Some(3).as_ref());
}

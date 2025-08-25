use error::*;

#[test]
fn test_div() {
    assert_eq!(div(1, 0), Err(MathError::DivByZero));
}

#[test]
fn test_get() {
    let v = vec![1, 2, 3];
    assert_eq!(get(&v, 0, 0), 1);
    assert_eq!(get(&v, 1, 0), 2);
    assert_eq!(get(&v, 2, 0), 3);
    assert_eq!(get(&v, 3, 0), 0);
}

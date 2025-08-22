use function::*;

#[test]
fn test_mul() {
    assert_eq!(mul(1, 0), 0);
    assert_eq!(mul(0, 1), 0);
    assert_eq!(mul(10, 2), 20);
    assert_eq!(mul(13, 7), 91);
}

#[test]
fn test_div() {
    assert_eq!(div(0, 1), 0);
    assert_eq!(div(10, 2), 5);
    assert_eq!(div(13, 7), 1);
}

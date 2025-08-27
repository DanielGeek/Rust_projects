use method::*;

#[test]
fn test_new() {
    let rect = Rectangle::new(100, 200);
    assert_eq!(rect.top, 0);
    assert_eq!(rect.left, 0);
    assert_eq!(rect.width, 100);
    assert_eq!(rect.height, 200);
}

#[test]
fn test_move_to() {
    let mut rect = Rectangle::new(100, 200);
    rect.move_to(10, 20);
    assert_eq!(rect.top, 10);
    assert_eq!(rect.left, 20);
    assert_eq!(rect.width, 100);
    assert_eq!(rect.height, 200);
}

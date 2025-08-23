use for_loop::*;

#[test]
fn test_sum() {
    assert_eq!(sum(vec![]), 0);
    assert_eq!(sum(vec![1, 2, 3]), 6);
    assert_eq!(sum(vec![1, -1, 1, -1]), 0);
}

#[test]
fn test_fill() {
    let v = fill(0, 10);
    assert_eq!(v.len(), 10);
    for i in v {
        assert_eq!(i, 0);
    }

    let v = fill(1, 11);
    assert_eq!(v.len(), 11);
    for i in v {
        assert_eq!(i, 1);
    }
}

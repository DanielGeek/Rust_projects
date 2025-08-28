use trait_bound::*;

#[test]
fn test_min() {
    assert_eq!(min(1u32, 2u32), 1u32);
    assert_eq!(min(0i32, -1i32), -1i32);
    assert_eq!(min(0.0, 3.0), 0.0);
    assert_eq!(min('a', 'b'), 'a');
}

#[test]
fn test_zip() {
    let a: Vec<u32> = vec![1, 2, 3];
    let b: Vec<i32> = vec![-2, -1, 0, 1, 2];
    assert_eq!(zip(a, b), vec![(1, -2), (2, -1), (3, 0)]);

    let a: Vec<u32> = vec![1, 2, 3, 4];
    let b: Vec<&str> = vec!["a", "b", "c"];
    assert_eq!(zip(a, b), vec![(1, "a"), (2, "b"), (3, "c")]);
}

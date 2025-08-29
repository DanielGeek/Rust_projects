use iterator_adaptors::*;
use std::collections::HashMap;

#[test]
fn test_filter_non_zero() {
    let v = vec![0, 1, 2, 0, 3, 4, 0];
    assert_eq!(filter_non_zero(v), vec![1, 2, 3, 4]);
}

#[test]
fn test_to_string() {
    let v: Vec<String> = to_string(vec!["a", "b", "c"]);
    assert_eq!(v.len(), 3);
    assert_eq!(v[0], "a");
    assert_eq!(v[1], "b");
    assert_eq!(v[2], "c");
}

#[test]
fn test_to_hash_map() {
    let v = vec![
        ("a".to_string(), 1),
        ("b".to_string(), 2),
        ("c".to_string(), 3),
    ];
    let h: HashMap<String, u32> = to_hash_map(v);
    assert_eq!(h.len(), 3);
    assert_eq!(h.get("a"), Some(&1));
    assert_eq!(h.get("b"), Some(&2));
    assert_eq!(h.get("c"), Some(&3));
}

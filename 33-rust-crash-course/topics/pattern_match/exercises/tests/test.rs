use pattern_match::*;

#[test]
fn test_num_to_string() {
    assert_eq!(num_to_string(0), "zero");
    assert_eq!(num_to_string(1), "one");
    assert_eq!(num_to_string(2), "two");
    assert_eq!(num_to_string(3), "three");
    assert_eq!(num_to_string(4), "other");
    assert_eq!(num_to_string(5), "other");
}

#[test]
fn test_unwrap_or_default() {
    assert_eq!(unwrap_or_default(Some(1), 0), 1);
    assert_eq!(unwrap_or_default(Some(2), 0), 2);
    assert_eq!(unwrap_or_default(None, 0), 0);
}

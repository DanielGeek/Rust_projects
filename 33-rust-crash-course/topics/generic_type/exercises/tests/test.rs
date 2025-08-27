use generic_type::*;

#[test]
fn test_first() {
    assert_eq!(first(('a', false)), 'a');
    assert_eq!(first((1u32, 0i32)), 1u32);
}

#[test]
fn test_last() {
    assert_eq!(last(('a', "hello")), "hello");
    assert_eq!(last((1u32, 0i32)), 0i32);
}

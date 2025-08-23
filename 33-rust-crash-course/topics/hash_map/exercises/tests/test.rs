use hash_map::*;

#[test]
fn test_init() {
    let addr = "0x123".to_string();
    let map = init(addr, 100);
    assert_eq!(map.len(), 1);
    assert_eq!(map.get("0x123"), Some(100).as_ref());
}

use unwrap::*;

#[test]
fn test_parse_and_add() {
    assert_eq!(parse_and_add("1", "2"), 3);
}

#[test]
#[should_panic(expected = "Failed to parse variable")]
fn test_parse_and_add_panic_1() {
    parse_and_add("a", "2");
}

#[test]
#[should_panic(expected = "Failed to parse variable")]
fn test_parse_and_add_panic_2() {
    parse_and_add("1", "b");
}

#[test]
fn test_unwrap_and_add() {
    let x = Some(1);
    let y = Some(2);
    assert_eq!(unwrap_and_add(x, y), 3);
}

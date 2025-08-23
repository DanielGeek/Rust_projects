use string::*;

#[test]
fn test_hello() {
    assert_eq!(hello(), "Hello Rust");
}

#[test]
fn test_greet() {
    assert_eq!(greet("Cyfrin"), "Hello Cyfrin");
}

#[test]
fn test_append() {
    let s = "Rust".to_string();
    assert_eq!(append(s), "Rust!");
}

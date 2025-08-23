use enum_type::*;

#[test]
fn test_color() {
    assert_eq!(Color::Red, Color::Red);
    assert!(Color::Green != Color::Blue);

    // Check if it compiles
    let _ = Color::Rgba(0, 255, 0, 0.5);
}

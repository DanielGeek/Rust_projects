use trait_basic::*;

#[test]
fn test_tester() {
    let foundry = Foundry {
        version: "0.3.0".to_string(),
    };
    let cargo = Cargo {
        version: "1.87.0".to_string(),
    };

    assert_eq!(foundry.test("hello.sol"), "forge test hello.sol");
    assert_eq!(cargo.test("hello.rs"), "cargo test hello.rs");

    assert_eq!(test(&foundry, "hello.sol"), "forge test hello.sol");
    assert_eq!(test(&cargo, "hello.rs"), "cargo test hello.rs");
}

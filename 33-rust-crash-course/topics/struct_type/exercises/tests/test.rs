use struct_type::*;

#[test]
fn test_account() {
    // Check if compiles
    let _ = Account {
        address: "0x123".to_string(),
        balance: 0,
    };

    let addr = "0x123".to_string();
    let account = new(addr.clone());
    assert_eq!(account.address, addr);
    assert_eq!(account.balance, 0);
}

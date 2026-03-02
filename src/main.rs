mod balances;

fn main() {
    println!("Hello, world!");
}

#[test]
fn init_balances() {
    let mut balances = balances::Pallet::new();

    assert_eq!(balances.balance(&"alice".to_string()), 0);
    balances.set_balance(&"alice".to_string(), 100);
    assert_eq!(balances.balance(&"alice".to_string()), 100);
    assert_eq!(balances.balance(&"bob".to_string()), 0);

}

#[test]
fn transfer_balance() {
    let mut balances = balances::Pallet::new();

    assert_eq!(
        balances.transfer("alice".to_string(), "bob".to_string(), 100),
        Err("Not enough funds")
    );
    balances.set_balance(&"alice".to_string(), 100);
    assert_eq!(
        balances.transfer("alice".to_string(), "bob".to_string(), 90),
        Ok(())
    );
    assert_eq!(balances.balance(&"alice".to_string()), 10);
    assert_eq!(balances.balance(&"bob".to_string()), 90);

    assert_eq!(
        balances.transfer("alice".to_string(), "bob".to_string(), 100),
        Err("Not enough funds")
    );
}

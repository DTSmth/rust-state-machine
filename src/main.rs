mod balances;
mod system;

#[derive(Debug)]
pub  struct Runtime {
    system: system::Pallet,
    balances: balances::Pallet,
}

impl Runtime {
    fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
        }
    }
}

fn main() {
    println!("Hello, world!");
    let mut runtime = Runtime::new();
    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();

    runtime.balances.set_balance(&alice, 100);
    runtime.system.inc_block_number();
    assert_eq!(runtime.system.block_number(), 1);

    runtime.system.inc_nonce(&"alice".to_string());
    let _res = runtime.balances.transfer(alice.clone(), bob, 30).map_err(|e| eprintln!("{e}"));

    runtime.system.inc_nonce(&alice);
    let _res = runtime.balances.transfer(alice, charlie, 20).map_err(|e| eprintln!("{e}"));

    println!("{runtime:#?}")

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

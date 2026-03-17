mod balances;
mod system;
mod support;
mod proof_of_existence;

use crate::balances::Call;
use crate::support::Dispatch;
use crate::types::{AccountId, Balance};

mod types {
    pub type AccountId = String;
    pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
    pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
    pub type Header = crate::support::Header<BlockNumber>;
    pub type Block = crate::support::Block<Header, Extrinsic>;
    pub type Content = String;
}

// These are all the calls which are exposed to the world.
// Note that it is just an accumulation of the calls exposed by each module.


#[derive(Debug)]
#[macros::runtime]
pub struct Runtime {
    system: system::Pallet<Self>,
    balances: balances::Pallet<Self>,
    proof_of_existence: proof_of_existence::Pallet<Self>,
}

impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
    type Balance = types::Balance;
}

impl proof_of_existence::Config for Runtime {
    type Content = types::Content;
}


fn main() {
    println!("Hello, world!");
    let mut runtime = Runtime::new();
    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();

    runtime.balances.set_balance(&alice, 100);

    let block_1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::balances(balances::Call::transfer {to: bob.clone(), amount: 30}),
            },
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::balances(balances::Call::transfer {to: charlie, amount: 20})
            },
        ],
    };

    let block_2 = types::Block {
        header: support::Header { block_number: 2},
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
                    claim: "Hello world".to_string(),
                })
            },
            support::Extrinsic {
                caller: bob.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
                    claim: "Hello world".to_string(),
                })
            }
        ],
    };

    let block_3 = types::Block {
        header: support::Header { block_number: 3},
        extrinsics: vec![
            support::Extrinsic {
                caller: alice,
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::revoke_claim {
                    claim: "Hello world".to_string(),
                }),
            },
            support::Extrinsic {
                caller: bob,
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
                    claim: "Hello world".to_string(),
                })
            },
        ],
    };

    runtime.execute_block(block_1).expect("invalid block");
    runtime.execute_block(block_2).expect("invalid block");
    runtime.execute_block(block_3).expect("invalid block");


    println!("{runtime:#?}")

}

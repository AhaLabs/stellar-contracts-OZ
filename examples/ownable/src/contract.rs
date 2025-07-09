//! Ownable Example Contract.
//!
//! Demonstrates an example usage of `ownable` module by
//! implementing `#[only_owner]` macro on a sensitive function.

use soroban_sdk::{contract, contractimpl, contracttype, derive_contract, Address, Env};
use stellar_ownable::Ownable;

#[contracttype]
pub enum DataKey {
    Owner,
    Counter,
}

#[contract]
#[derive_contract(Ownable)]
pub struct ExampleContract;

#[contractimpl]
impl ExampleContract {
    pub fn __constructor(e: &Env, owner: Address) {
        Self::set_owner(e, &owner);
        e.storage().instance().set(&DataKey::Counter, &0);
    }

    pub fn increment(e: &Env) -> i32 {
        Self::only_owner(e);
        let mut counter: i32 =
            e.storage().instance().get(&DataKey::Counter).expect("counter should be set");

        counter += 1;

        e.storage().instance().set(&DataKey::Counter, &counter);

        counter
    }
}

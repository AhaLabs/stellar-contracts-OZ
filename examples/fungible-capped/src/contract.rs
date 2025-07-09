//! Capped Example Contract.
//!
//! Demonstrates an example usage of `capped` module by
//! implementing a capped mint mechanism, and setting the maximum supply
//! at the constructor.
//!
//! **IMPORTANT**: this example is for demonstration purposes, and authorization
//! is not taken into consideration

use soroban_sdk::{contract, contractimpl, Address, Env, String};
use stellar_fungible::{
    capped::{check_cap, set_cap},
    Base, FungibleToken,
};

#[contract]
#[derive_contract(FungibleToken)]
pub struct ExampleContract;

#[contractimpl]
impl ExampleContract {
    pub fn __constructor(e: &Env, cap: i128) {
        set_cap(e, cap);
    }

    pub fn mint(e: &Env, account: Address, amount: i128) {
        check_cap(e, amount);
        Base::mint(e, &account, amount);
    }
}

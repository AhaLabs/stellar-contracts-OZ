//! Fungible Pausable Example Contract.

//! This contract showcases how to integrate various OpenZeppelin modules to
//! build a fully SEP-41-compliant fungible token. It includes essential
//! features such as an emergency stop mechanism and controlled token minting by
//! the owner.
//!
//! To meet SEP-41 compliance, the contract must implement both
//! [`stellar_fungible::fungible::FungibleToken`] and
//! [`stellar_fungible::burnable::FungibleBurnable`].

use soroban_sdk::{
    contract, contracterror, contractimpl, derive_contract, panic_with_error, symbol_short, Address, Env, String, Symbol
};
use stellar_fungible::{FungibleBurnable, impl_token_interface, Base, FungibleToken};
use stellar_pausable::{Pausable, PausableExt};
use stellar_ownable::{Ownable, OwnableExt};


#[contract]
#[derive_contract(
    Ownable,
    FungibleToken(ext = PausableExt),
    FungibleBurnable(ext = PausableExt),
    Pausable(ext = OwnableExt),
)]
pub struct ExampleContract;

#[contractimpl]
impl ExampleContract {
    pub fn __constructor(e: &Env, owner: Address, initial_supply: i128) {
        Base::set_metadata(e, 18, String::from_str(e, "My Token"), String::from_str(e, "TKN"));
        Self::set_owner(e, owner);
        Base::mint(e, &owner, initial_supply);
    }

    pub fn mint(e: &Env, account: Address, amount: i128) {
        Self::when_not_paused(e);
        Self::enforce_owner_auth(e);
        Base::mint(e, &account, amount);
    }
}


// NOTE: if your contract implements `FungibleToken` and `FungibleBurnable`,
// and you also want your contract to implement
// `soroban_sdk::token::TokenInterface`, you can use the `impl_token_interface!`
// macro to generate the boilerplate implementation.
impl_token_interface!(ExampleContract);

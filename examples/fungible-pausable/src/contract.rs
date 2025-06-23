//! Fungible Pausable Example Contract.

//! This contract showcases how to integrate various OpenZeppelin modules to
//! build a fully SEP-41-compliant fungible token. It includes essential
//! features such as an emergency stop mechanism and controlled token minting by
//! the owner.
//!
//! To meet SEP-41 compliance, the contract must implement both
//! [`stellar_fungible::fungible::FungibleToken`] and
//! [`stellar_fungible::burnable::FungibleBurnable`].

use soroban_sdk::{contract, contracterror, contractimpl, Address, Env, String};
use stellar_contract_utils::{
    derive_contract_base, derive_contract_extension, extensions::blocklist::BlockListImpl, Admin,
    Administratable, Base, FungibleBlockList, FungibleBurnable, FungibleMintable, FungibleToken,
    Pausable, PausableToken, PauseableBase,
};
// use stellar_ft::{Base, FungibleBurnable, FungibleMintable, FungibleToken};
// use stellar_pausable::AdminPausable;

#[contract]
#[derive_contract_base(Administratable)]
// #[derive_contract_extension(FungibleMintable, FungibleBurnable, FungibleToken)]
pub struct ExampleContract;

Pausable!(ExampleContract);
// FungibleBlockList!(ExampleContract, BlockListImpl<ExampleContract>);
impl FungibleToken for ExampleContract {
    type Impl = Base;
}
// FungibleToken!(ExampleContract, PausableToken<BlockListImpl<ExampleContract>>);
FungibleToken!(ExampleContract, PausableToken<ExampleContract>);

FungibleBurnable!(ExampleContract, PausableToken<ExampleContract>);
FungibleMintable!(ExampleContract, PausableToken<ExampleContract>);

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ExampleContractError {
    Unauthorized = 1,
}

#[contractimpl]
impl ExampleContract {
    pub fn __constructor(e: &Env, owner: Address, initial_supply: i128) {
        Self::set_admin(e, owner.clone());
        Base::set_metadata(e, 18, String::from_str(e, "My Token"), String::from_str(e, "TKN"));
        Base::mint(e, owner, initial_supply);
    }
}

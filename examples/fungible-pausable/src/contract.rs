//! Fungible Pausable Example Contract.

//! This contract showcases how to integrate various OpenZeppelin modules to
//! build a fully SEP-41-compliant fungible token. It includes essential
//! features such as an emergency stop mechanism and controlled token minting by
//! the owner.
//!
//! To meet SEP-41 compliance, the contract must implement both
//! [`stellar_fungible::fungible::FungibleToken`] and
//! [`stellar_fungible::burnable::FungibleBurnable`].

use soroban_sdk::{contract, contractimpl, Address, Env, String};
use stellar_contract_utils::{
    derive_contract,
    extensions::blocklist::{BlockList, FungibleBlockListExt},
    Admin, Administratable, AdministratableExt, Base, Empty, EmptyExt,
    FungibleBlockList, FungibleBurnable, FungibleMintable, FungibleToken, Pausable, PausableExt,
    PauseableBase, Upgradable
};

#[contract]
#[derive_contract(
    Administratable,
    Upgradable(
        ext = AdministratableExt,
    ),
    // Example Extension that does nothing
    Empty,
    Pausable(
        ext = AdministratableExt,
    ),
    FungibleBlockList(
        ext = AdministratableExt,
    ),
    FungibleMintable(
        ext = AdministratableExt,
        ext = PausableExt,
        ext = EmptyExt,
    ),
    FungibleBurnable(
        ext = PausableExt,
    ),
    FungibleToken(
        ext = FungibleBlockListExt, 
        ext = PausableExt
    ),
)]
pub struct ExampleContract;

#[contractimpl]
impl ExampleContract {
    pub fn __constructor(e: &Env, owner: Address, initial_supply: i128) {
        Self::set_admin(e, owner.clone());
        Base::set_metadata(e, 18, String::from_str(e, "My Token"), String::from_str(e, "TKN"));
        Base::mint(e, owner, initial_supply);
    }
}

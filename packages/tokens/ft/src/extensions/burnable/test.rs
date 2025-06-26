#![cfg(test)]

extern crate std;

use soroban_sdk::{contract, testutils::Address as _, Address, Env};
use stellar_event_assertion::EventAssertion;

use super::FungibleBurnable;
use crate::{fungible::FungibleToken, Base};
use super::super::mintable::FungibleMintable;

#[contract]
struct MockContract;

#[test]
fn burn_works() {
    let e = Env::default();
    e.mock_all_auths();
    let address = e.register(MockContract, ());
    let account = Address::generate(&e);
    e.as_contract(&address, || {
        Base::mint(&e, account.clone(), 100);
        Base::burn(&e, account.clone(), 50);
        assert_eq!(Base::balance(&e, account.clone()), 50);
        assert_eq!(Base::total_supply(&e), 50);

        let mut event_assert = EventAssertion::new(&e, address.clone());
        event_assert.assert_event_count(2);
        event_assert.assert_fungible_mint(&account, 100);
        event_assert.assert_fungible_burn(&account, 50);
    });
}

#[test]
fn burn_with_allowance_works() {
    let e = Env::default();
    e.mock_all_auths();
    let address = e.register(MockContract, ());
    let owner = Address::generate(&e);
    let spender = Address::generate(&e);
    e.as_contract(&address, || {
        Base::mint(&e, owner.clone(), 100);
        Base::approve(&e, owner.clone(), spender.clone(), 30, 1000);
        Base::burn_from(&e, spender.clone(), owner.clone(), 30);
        assert_eq!(Base::balance(&e, owner.clone()), 70);
        assert_eq!(Base::balance(&e, spender.clone()), 0);
        assert_eq!(Base::total_supply(&e), 70);

        let mut event_assert = EventAssertion::new(&e, address.clone());
        event_assert.assert_event_count(3);
        event_assert.assert_fungible_mint(&owner, 100);
        event_assert.assert_fungible_approve(&owner, &spender, 30, 1000);
        event_assert.assert_fungible_burn(&owner, 30);
    });
}

#[test]
#[should_panic(expected = "Error(Contract, #100)")]
fn burn_with_insufficient_balance_panics() {
    let e = Env::default();
    e.mock_all_auths();
    let address = e.register(MockContract, ());
    let account = Address::generate(&e);
    e.as_contract(&address, || {
        Base::mint(&e, account.clone(), 100);
        assert_eq!(Base::balance(&e, account.clone()), 100);
        assert_eq!(Base::total_supply(&e), 100);
        Base::burn(&e, account, 101);
    });
}

#[test]
#[should_panic(expected = "Error(Contract, #101)")]
fn burn_with_no_allowance_panics() {
    let e = Env::default();
    e.mock_all_auths();
    let address = e.register(MockContract, ());
    let owner = Address::generate(&e);
    let spender = Address::generate(&e);
    e.as_contract(&address, || {
        Base::mint(&e, owner.clone(), 100);
        assert_eq!(Base::balance(&e, owner.clone()), 100);
        assert_eq!(Base::total_supply(&e), 100);
        Base::burn_from(&e, spender, owner, 50);
    });
}

#[test]
#[should_panic(expected = "Error(Contract, #101)")]
fn burn_with_insufficient_allowance_panics() {
    let e = Env::default();
    e.mock_all_auths();
    let address = e.register(MockContract, ());
    let owner = Address::generate(&e);
    let spender = Address::generate(&e);
    e.as_contract(&address, || {
        Base::mint(&e, owner.clone(), 100);
        Base::approve(&e, owner.clone(), spender.clone(), 50, 100);
        assert_eq!(Base::allowance(&e, owner.clone(), spender.clone()), 50);
        assert_eq!(Base::balance(&e, owner.clone()), 100);
        assert_eq!(Base::total_supply(&e), 100);
        Base::burn_from(&e, spender, owner, 60);
    });
}

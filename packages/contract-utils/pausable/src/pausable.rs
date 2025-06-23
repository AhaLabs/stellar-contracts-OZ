use admin_sep::{AdminExt, Administratable};
use contract_trait_macro::contracttrait;
use soroban_sdk::{contracterror, symbol_short, Address, Env};

use crate::{BaseToken, PauseableBase};

#[contracttrait(default = PauseableBase)]
pub trait Pausable {
    fn paused(e: &Env) -> bool;

    fn pause(e: &Env, caller: Address);

    fn unpause(e: &Env, caller: Address);
}

// impl<T> Pausable for T
// where
//     T:  Administratable,
// {
//     type Impl = PauseableBase;

//     fn pause(e: &soroban_sdk::Env, operator: soroban_sdk::Address) {
//         Self::require_admin(e);
//         Self::Impl::pause(e, operator);
//     }

//     fn unpause(e: &soroban_sdk::Env, operator: soroban_sdk::Address) {
//         Self::require_admin(e);
//         Self::Impl::unpause(e, operator);
//     }
// }

pub trait PauseChecker {
    fn when_not_paused(e: &Env);

    fn when_paused(e: &Env);
}

impl<T> PauseChecker for T
where
    T: Pausable,
{
    fn when_not_paused(e: &Env) {
        if T::paused(e) {
            soroban_sdk::panic_with_error!(e, PausableError::EnforcedPause);
        }
    }

    fn when_paused(e: &Env) {
        if !T::paused(e) {
            soroban_sdk::panic_with_error!(e, PausableError::ExpectedPause);
        }
    }
}

// ################## ERRORS ##################

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum PausableError {
    /// The operation failed because the contract is paused.
    EnforcedPause = 1000,
    /// The operation failed because the contract is not paused.
    ExpectedPause = 1001,
}

// ################## EVENTS ##################

/// Emits an event when `Paused` state is triggered.
///
/// # Arguments
///
/// * `e` - Access to Soroban environment.
/// * `caller` - The address of the caller.
///
/// # Events
///
/// * topics - `["paused"]`
/// * data - `[caller: Address]`
pub fn emit_paused(e: &Env, caller: &Address) {
    let topics = (symbol_short!("paused"),);
    e.events().publish(topics, caller)
}

/// Emits an event when `Unpaused` state is triggered.
///
/// # Arguments
///
/// * `e` - Access to Soroban environment.
/// * `caller` - The address of the caller.
///
/// # Events
///
/// * topics - `["unpaused"]`
/// * data - `[caller: Address]`
pub fn emit_unpaused(e: &Env, caller: &Address) {
    let topics = (symbol_short!("unpaused"),);
    e.events().publish(topics, caller)
}

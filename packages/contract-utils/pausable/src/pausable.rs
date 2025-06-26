use admin_sep::contracttrait;
use soroban_sdk::{contracterror, symbol_short, Address, Env};


/// The `PauseableBase` trait provides the default implementation for the `Pausable` trait.
/// It requires an extension to be implemented, which is expected to provide the authorization logic
/// for pausing and unpausing the contract.
#[contracttrait(
    default = PauseableBase, 
    extension_required = true, 
    is_extension = true
)]
pub trait Pausable {
    /// Returns true if the contract is paused, and false otherwise.
    fn paused(e: &Env) -> bool;

    /// Authorized function to pause the contract.
    fn pause(e: &Env, caller: soroban_sdk::Address);

    /// Authorized function to unpause the contract.
    fn unpause(e: &Env, caller: soroban_sdk::Address);
}

pub trait PauseChecker {
    fn when_not_paused(e: &Env);

    fn when_paused(e: &Env);
}

impl<T: Pausable> PauseChecker for T
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

use soroban_sdk::{Address, Env};

use crate::{burnable::FungibleBurnable, extensions::burnable::emit_burn, storage::EventExt, Base};


impl super::FungibleBurnable for Base {
    type Impl = EventExt<Base>;
    /// Destroys `amount` of tokens from `from`. Updates the total
    /// supply accordingly.
    ///
    /// # Arguments
    ///
    /// * `e` - Access to the Soroban environment.
    /// * `from` - The account whose tokens are destroyed.
    /// * `amount` - The amount of tokens to burn.
    ///
    /// # Errors
    ///
    /// * refer to [`Base::update`] errors.
    ///
    /// # Events
    ///
    /// * topics - `["burn", from: Address]`
    /// * data - `[amount: i128]`
    ///
    /// # Notes
    ///
    /// Authorization for `from` is required.
    fn burn(e: &Env, from: &Address, amount: i128) {
        from.require_auth();
        Base::update(e, Some(from), None, amount);
    }

    /// Destroys `amount` of tokens from `from` using the allowance mechanism.
    /// `amount` is then deducted from `spender` allowance.
    /// Updates the total supply accordingly.
    ///
    /// # Arguments
    ///
    /// * `e` - Access to the Soroban environment.
    /// * `spender` - The address authorizing the transfer, and having its
    ///   allowance.
    /// * `from` - The account whose tokens are destroyed.
    /// * `amount` - The amount of tokens to burn.
    ///
    /// # Errors
    ///
    /// * refer to [`Base::spend_allowance`] errors.
    /// * refer to [`Base::update`] errors.
    ///
    /// # Events
    ///
    /// * topics - `["burn", from: Address]`
    /// * data - `[amount: i128]`
    ///
    /// # Notes
    ///
    /// Authorization for `spender` is required.
    fn burn_from(e: &Env, spender: &Address, from: &Address, amount: i128) {
        spender.require_auth();
        Base::spend_allowance(e, from, spender, amount);
        Base::update(e, Some(from), None, amount);
    }
}

impl<T: FungibleBurnable> FungibleBurnable for EventExt<T> {
    type Impl = T::Impl;

    fn burn(e: &Env, from: &Address, amount: i128) {
        T::burn(e, from, amount);
        emit_burn(e, from, amount);
    }

    fn burn_from(e: &Env, spender: &Address, from: &Address, amount: i128) {
        T::burn_from(e, spender, from, amount);
        emit_burn(e, from, amount);
    }
}
//! # Fungible Token Contract Module.
//!
//! Implements utilities for handling fungible tokens in a Soroban contract.
//!
//! This module provides essential storage functionalities required for managing
//! balances, allowances, and total supply of fungible tokens.
//!
//! ## Design Overview
//!
//! This module is structured to provide flexibility to developers by splitting
//! functionalities into higher-level and lower-level operations:
//!
//! - **High-Level Functions**: These include all necessary checks,
//!   verifications, authorizations, state-changing logic, and event emissions.
//!   They simplify usage by handling core logic securely. Users can directly
//!   call these functions for typical token operations without worrying about
//!   implementation details.
//!
//! - **Low-Level Functions**: These offer granular control for developers who
//!   need to compose their own workflows. Such functions expose internal
//!   mechanisms and require the caller to handle verifications and
//!   authorizations manually.
//!
//! By offering this dual-layered approach, developers can choose between
//! convenience and customization, depending on their project requirements.
//!
//! ## Structure
//!
//! The base module includes:
//!
//! - Total supply management
//! - Transfers and allowances
//!
//! The following optional extensions are available:
//!
//! - Metadata: Provides additional information about the token, such as name,
//!   symbol, and decimals.
//! - Burnable: Enables token holders to destroy their tokens, reducing the
//!   total supply.
//! - Capped: Enables the contract to set a maximum limit on the total supply.
//!
//! ## Compatibility and Compliance
//!
//! The module is designed to ensure full compatibility with SEP-0041. It also
//! closely mirrors the Ethereum ERC-20 standard, facilitating cross-ecosystem
//! familiarity and ease of use.
//!
//! Developers aiming to create SEP-41-compliant tokens can leverage the
//! `soroban_sdk::token::TokenInterface` trait available in the "soroban-sdk"
//! crate. By implementing `TokenInterface` using the helper functions provided
//! in this library, they can ensure a secure and standardized implementation.
//! Alternatively, developers can combine the implementation of both the
//! [`FungibleToken`] and [`burnable::FungibleBurnable`] traits to create tokens
//! that adhere to SEP-41 while providing greater control and extensibility.
//!
//! ## Notes for Developers
//!
//! - **Security Considerations**: While high-level functions handle necessary
//!   checks, users of low-level functions must take extra care to ensure
//!   correctness and security.
//! - **Composable Design**: The modular structure encourages developers to
//!   extend functionality by combining provided primitives or creating custom
//!   extensions.
//! - **TTL management**: This library handles the TTL of only `temporary` and
//!   `persistent` storage entries declared by the library. The `instance` TTL
//!   management is left to the implementor due to flexibility. The library
//!   exposes the sane default values for extending the TTL:
//!   `INSTANCE_TTL_THRESHOLD` and `INSTANCE_EXTEND_AMOUNT`.
#![no_std]

pub mod extensions;
mod fungible;
// mod impl_token_interface_macro;
// mod overrides;
pub(crate) mod storage;
// mod utils;

use admin_sep::{AdminExt, Administratable};

pub use extensions::blocklist::FungibleBlockList;
pub use extensions::burnable::FungibleBurnable;
pub use extensions::mintable::FungibleMintable;
// pub use extensions::{allowlist, blocklist, burnable, capped};
pub use fungible::{emit_approve, emit_transfer, FungibleToken, FungibleTokenError};
use stellar_pausable::{Pausable, PausableToken, PauseChecker};
pub use storage::{AllowanceData, AllowanceKey, Base, StorageKey};
// pub use utils::{sac_admin_generic, sac_admin_wrapper};

impl<T: FungibleToken + Pausable> FungibleToken for PausableToken<T> {
    type Impl = T;
    fn transfer(
        e: &soroban_sdk::Env,
        from: soroban_sdk::Address,
        to: soroban_sdk::Address,
        amount: i128,
    ) {
        Self::Impl::when_not_paused(e);
        Self::Impl::transfer(e, from, to, amount);
    }

    fn transfer_from(
        e: &soroban_sdk::Env,
        spender: soroban_sdk::Address,
        from: soroban_sdk::Address,
        to: soroban_sdk::Address,
        amount: i128,
    ) {
        Self::Impl::when_not_paused(e);
        Self::Impl::transfer_from(e, spender, from, to, amount);
    }
}

impl<T> FungibleBurnable for PausableToken<T>
where
    T: Administratable,
{
    type Impl = Base;
    fn burn(e: &soroban_sdk::Env, from: soroban_sdk::Address, amount: i128) {
        Self::when_not_paused(e);
        Base::burn(e, from, amount);
    }
    fn burn_from(
        e: &soroban_sdk::Env,
        spender: soroban_sdk::Address,
        from: soroban_sdk::Address,
        amount: i128,
    ) {
        Self::when_not_paused(e);
        Base::burn_from(e, spender, from, amount);
    }
}

// Mintable

impl<T> FungibleMintable for PausableToken<T>
where
    T: Administratable,
{
    type Impl = Base;
    fn mint(e: &soroban_sdk::Env, to: soroban_sdk::Address, amount: i128) {
        Self::when_not_paused(e);
        T::require_admin(e);
        Self::Impl::mint(e, to, amount);
    }
}

// impl<T> FungibleMintable for T
// where
//     T: BaseToken + Administratable,
// {
//     type Impl = Base;
//     fn mint(e: &soroban_sdk::Env, to: soroban_sdk::Address, amount: i128) {
//         Self::require_admin(e);
//         Base::mint(e, to, amount);
//     }
// }

mod test;

//! # Soroban Smart Accounts
//!
//! A flexible and modular smart account framework for Soroban that enables
//! advanced authentication and authorization patterns through composable rules,
//! signers, and policies.
#![no_std]

// Ensure soroban-sdk's panic handler is linked for cdylib builds.
extern crate soroban_sdk;

#[cfg(any(
    feature = "smart-account",
    feature = "simple-threshold",
    feature = "weighted-threshold",
    feature = "spending-limit"
))]
pub mod policies;
#[cfg(feature = "smart-account")]
pub mod smart_account;
#[cfg(any(feature = "smart-account", feature = "ed25519", feature = "webauthn"))]
pub mod verifiers;

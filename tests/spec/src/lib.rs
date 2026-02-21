#![no_std]

// #[cfg(feature = "wasms")]
mod test {
    mod fungible_merkle_airdrop {
        soroban_sdk::contractimport!(
            file = "../../target/wasm32v1-none/release/fungible_merkle_airdrop_example.wasm"
        );
    }
    mod fungible_allowlist {
        soroban_sdk::contractimport!(
            file = "../../target/wasm32v1-none/release/fungible_allowlist_example.wasm"
        );
    }

    mod fungible_blocklist {
        soroban_sdk::contractimport!(
            file = "../../target/wasm32v1-none/release/fungible_blocklist_example.wasm"
        );
    }

    mod fungible_capped {
        soroban_sdk::contractimport!(
            file = "../../target/wasm32v1-none/release/fungible_capped_example.wasm"
        );
    }

    mod fungible_pausable {
        soroban_sdk::contractimport!(
            file = "../../target/wasm32v1-none/release/fungible_pausable_example.wasm"
        );
    }

    mod fungible_vault {
        soroban_sdk::contractimport!(
            file = "../../target/wasm32v1-none/release/fungible_vault_example.wasm"
        );
    }

    mod fungible_votes {
        soroban_sdk::contractimport!(
            file = "../../target/wasm32v1-none/release/fungible_votes_example.wasm"
        );
    }

    mod fee_forwarder_permissioned {
        soroban_sdk::contractimport!(
            file = "../../target/wasm32v1-none/release/fee_forwarder_permissioned_example.wasm"
        );
    }

    mod fee_forwarder_permissionless {
        soroban_sdk::contractimport!(
            file = "../../target/wasm32v1-none/release/fee_forwarder_permissionless_example.wasm"
        );
    }

    mod merkle_voting {
        soroban_sdk::contractimport!(
            file = "../../target/wasm32v1-none/release/merkle_voting_example.wasm"
        );
    }

    mod multisig_account {
        pub use soroban_sdk::auth::Context;
        soroban_sdk::contractimport!(
            file = "../../target/wasm32v1-none/release/multisig_account_example.wasm"
        );
    }

    mod multisig_ed25519_verifier {
        soroban_sdk::contractimport!(
            file = "../../target/wasm32v1-none/release/multisig_ed25519_verifier_example.wasm"
        );
    }

    mod multisig_spending_limit_policy {
        pub use soroban_sdk::auth::Context;
        soroban_sdk::contractimport!(
            file = "../../target/wasm32v1-none/release/multisig_spending_limit_policy_example.wasm"
        );
    }

    mod multisig_threshold_policy {
        pub use soroban_sdk::auth::Context;
        soroban_sdk::contractimport!(
            file = "../../target/wasm32v1-none/release/multisig_threshold_policy_example.wasm"
        );
    }

    mod multisig_webauthn_verifier {
        soroban_sdk::contractimport!(
            file = "../../target/wasm32v1-none/release/multisig_webauthn_verifier_example.wasm"
        );
    }

    mod nft_access_control {
        soroban_sdk::contractimport!(
            file = "../../target/wasm32v1-none/release/nft_access_control_example.wasm"
        );
    }

    mod nft_consecutive {
        soroban_sdk::contractimport!(
            file = "../../target/wasm32v1-none/release/nft_consecutive_example.wasm"
        );
    }

    mod nft_enumerable {
        soroban_sdk::contractimport!(
            file = "../../target/wasm32v1-none/release/nft_enumerable_example.wasm"
        );
    }

    mod nft_royalties {
        soroban_sdk::contractimport!(
            file = "../../target/wasm32v1-none/release/nft_royalties_example.wasm"
        );
    }

    mod nft_sequential_minting {
        soroban_sdk::contractimport!(
            file = "../../target/wasm32v1-none/release/nft_sequential_minting_example.wasm"
        );
    }

    mod ownable {
        soroban_sdk::contractimport!(
            file = "../../target/wasm32v1-none/release/ownable_example.wasm"
        );
    }

    mod pausable {
        soroban_sdk::contractimport!(
            file = "../../target/wasm32v1-none/release/pausable_example.wasm"
        );
    }

    mod sac_admin_generic {
        pub use soroban_sdk::auth::Context;
        soroban_sdk::contractimport!(
            file = "../../target/wasm32v1-none/release/sac_admin_generic_example.wasm"
        );
    }

    mod sac_admin_wrapper {
        soroban_sdk::contractimport!(
            file = "../../target/wasm32v1-none/release/sac_admin_wrapper_example.wasm"
        );
    }

    mod timelock_controller {
        pub use soroban_sdk::auth::Context;
        soroban_sdk::contractimport!(
            file = "../../target/wasm32v1-none/release/timelock_controller_example.wasm"
        );
    }

    mod upgradeable_v1 {
        soroban_sdk::contractimport!(
            file = "../../target/wasm32v1-none/release/upgradeable_v1_example.wasm"
        );
    }

    mod upgradeable_v2 {
        soroban_sdk::contractimport!(
            file = "../../target/wasm32v1-none/release/upgradeable_v2_example.wasm"
        );
    }

    mod upgrader {
        soroban_sdk::contractimport!(
            file = "../../target/wasm32v1-none/release/upgrader_example.wasm"
        );
    }
}

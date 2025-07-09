//! Fungible AllowList Example Contract.

//! This contract showcases how to integrate the AllowList extension with a
//! SEP-41-compliant fungible token. It includes essential features such as
//! controlled token transfers by an admin who can allow or disallow specific
//! accounts.

use soroban_sdk::{contract, contractimpl, derive_contract, symbol_short, Address, Env, String};
use stellar_access_control::{self as access_control, AccessControl};
use stellar_access_control_macros::has_role;
use stellar_fungible::{
    Base, FungibleAllowList, FungibleAllowListExt, FungibleBurnable, FungibleToken,
};

#[contract]
#[derive_contract(
    AccessControl(default = MyAllowList),
    FungibleToken(ext = FungibleAllowListExt),
    FungibleAllowList(default = MyAllowList),
    FungibleBurnable(ext = FungibleAllowListExt),
)]
pub struct ExampleContract;

#[contractimpl]
impl ExampleContract {
    pub fn __constructor(e: &Env, admin: Address, manager: Address, initial_supply: i128) {
        Base::set_metadata(
            e,
            18,
            String::from_str(e, "AllowList Token"),
            String::from_str(e, "ALT"),
        );

        Self::set_admin(e, &admin);

        // create a role "manager" and grant it to `manager`
        access_control::grant_role_no_auth(e, &admin, &manager, &symbol_short!("manager"));

        // Allow the admin to transfer tokens
        <Self as FungibleAllowList>::allow_user(e, &admin, &admin);

        // Mint initial supply to the admin
        Base::mint(e, &admin, initial_supply);
    }
}

pub struct MyAllowList;

impl AccessControl for MyAllowList {
    type Impl = AccessControl!();
}

impl FungibleAllowList for MyAllowList {
    
    type Impl = FungibleAllowList!();

    #[has_role(operator, "manager")]
    fn allow_user(e: &Env, user: &Address, operator: &Address) {
        Self::Impl::allow_user(e, user)
    }

    #[has_role(operator, "manager")]
    fn disallow_user(e: &Env, user: &Address, operator: &Address) {
        Self::Impl::disallow_user(e, user)
    }
}
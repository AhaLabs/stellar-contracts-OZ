use soroban_sdk::{contract, contractimpl, derive_contract, symbol_short, Address, Env};
use stellar_access_control::{self as access_control, AccessControl};
use stellar_access_control_macros::{has_role, only_admin};
use stellar_fungible::{self as fungible, SACAdminWrapper};

#[contract]
#[derive_contract(
    SACAdminWrapper(default = MySACAdminWrapper),
    AccessControl(default = MySACAdminWrapper),
)]
pub struct ExampleContract;

#[contractimpl]
impl ExampleContract {
    pub fn __constructor(
        e: &Env,
        default_admin: Address,
        manager1: Address,
        manager2: Address,
        sac: Address,
    ) {
        access_control::set_admin(e, &default_admin);

        // create a role "manager" and grant it to `manager1`
        access_control::grant_role_no_auth(e, &default_admin, &manager1, &symbol_short!("manager"));

        // grant it to `manager2`
        access_control::grant_role_no_auth(e, &default_admin, &manager2, &symbol_short!("manager"));

        fungible::sac_admin_wrapper::set_sac_address(e, &sac);
    }
}

pub struct MySACAdminWrapper;

impl AccessControl for MySACAdminWrapper {
    type Impl = AccessControl!();
}


impl SACAdminWrapper for MySACAdminWrapper {
    type Impl =SACAdminWrapper!();
    #[only_admin]
    fn set_admin(e: Env, new_admin: &Address, _operator: &Address) {
        Self::Impl::set_admin(&e, &new_admin);
    }

    #[has_role(operator, "manager")]
    fn set_authorized(e: Env, id: &Address, authorize: bool, operator: &Address) {
        Self::Impl::set_authorized(&e, id, authorize);
    }

    #[has_role(operator, "manager")]
    fn mint(e: Env, to: &Address, amount: i128, operator: &Address) {
        Self::Impl::mint(e, to, amount, operator);
    }

    #[has_role(operator, "manager")]
    fn clawback(e: Env, from: &Address, amount: i128, operator: &Address) {
        Self::Impl::clawback(&e, &from, amount, operator);
    }
}

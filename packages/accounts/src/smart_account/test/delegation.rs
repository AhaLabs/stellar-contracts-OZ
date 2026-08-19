//! End-to-end CAP-0071 delegation: a smart account whose `__check_auth` is
//! `do_check_auth`, with a context rule whose only signer is
//! `Signer::Delegated(<custom account>)`. Enforced authorization drives the
//! host's delegated-auth tracker via a
//! `SorobanCredentials::AddressWithDelegates` entry — the delegate's
//! authentication shares this account's authorization entry instead of
//! requiring its own.
//!
//! Note: `env.try_invoke_contract_check_auth` cannot attach delegated signers,
//! so the flow is tested with `set_auths` + a wrapper call requiring the
//! account's auth (same approach as the sdk's own delegation test).

extern crate std;

use soroban_sdk::{
    auth::{Context, CustomAccountInterface},
    contract, contractimpl,
    crypto::Hash,
    map,
    testutils::Ledger,
    vec,
    xdr::{
        InvokeContractArgs, ScAddress, ScVal, SorobanAddressCredentials,
        SorobanAddressCredentialsWithDelegates, SorobanAuthorizationEntry,
        SorobanAuthorizedFunction, SorobanAuthorizedInvocation, SorobanCredentials,
        SorobanDelegateSignature, StringM, VecM,
    },
    Address, Bytes, Env, IntoVal, Map, String, TryFromVal, Val, Vec,
};

use crate::smart_account::{
    self,
    storage::{add_context_rule, AuthPayload, ContextRuleType, Signer},
    SmartAccountError,
};

// The account under test: authorization is exactly `do_check_auth`.
#[contract]
struct SmartAccountContract;

#[contractimpl]
impl CustomAccountInterface for SmartAccountContract {
    type Error = SmartAccountError;
    type Signature = AuthPayload;

    fn __check_auth(
        e: Env,
        signature_payload: Hash<32>,
        signatures: AuthPayload,
        auth_contexts: Vec<Context>,
    ) -> Result<(), SmartAccountError> {
        smart_account::do_check_auth(&e, &signature_payload, &signatures, &auth_contexts)
    }
}

// The delegate: a custom account that approves everything. Its authentication
// runs in its own `__check_auth` frame, forwarded by the host via
// `delegate_account_auth` — no signature material needed for this test.
#[contract]
struct ApproveAllAccount;

#[contractimpl]
impl CustomAccountInterface for ApproveAllAccount {
    type Error = SmartAccountError;
    type Signature = Val;

    fn __check_auth(
        _e: Env,
        _signature_payload: Hash<32>,
        _signature: Val,
        _auth_contexts: Vec<Context>,
    ) -> Result<(), SmartAccountError> {
        Ok(())
    }
}

#[contract]
struct Protected;

#[contractimpl]
impl Protected {
    pub fn protected(account: Address) {
        account.require_auth();
    }
}

struct Setup {
    env: Env,
    account: Address,
    delegate: Address,
    protected: Address,
    rule_id: u32,
}

fn setup() -> Setup {
    let env = Env::default();
    env.ledger().with_mut(|l| l.sequence_number = 10);

    let account = env.register(SmartAccountContract, ());
    let delegate = env.register(ApproveAllAccount, ());
    let protected = env.register(Protected, ());

    let rule = env.as_contract(&account, || {
        add_context_rule(
            &env,
            &ContextRuleType::CallContract(protected.clone()),
            &String::from_str(&env, "delegated"),
            None,
            &vec![&env, Signer::Delegated(delegate.clone())],
            &Map::new(&env),
        )
    });

    Setup { env, account, delegate, protected, rule_id: rule.id }
}

/// The account's credential signature: the AuthPayload selecting `rule_id`
/// and naming the delegated signer (with empty signature bytes — its
/// authentication is the host-forwarded delegation, not signature data).
fn auth_payload_scval(s: &Setup) -> ScVal {
    let payload = AuthPayload {
        signers: map![&s.env, (Signer::Delegated(s.delegate.clone()), Bytes::new(&s.env))],
        context_rule_ids: vec![&s.env, s.rule_id],
    };
    let payload_val: Val = payload.into_val(&s.env);
    ScVal::try_from_val(&s.env, &payload_val).unwrap()
}

fn root_invocation(s: &Setup) -> SorobanAuthorizedInvocation {
    let account_addr: ScAddress = s.account.clone().into();
    SorobanAuthorizedInvocation {
        function: SorobanAuthorizedFunction::ContractFn(InvokeContractArgs {
            contract_address: s.protected.clone().into(),
            function_name: StringM::try_from("protected").unwrap().into(),
            args: std::vec![ScVal::Address(account_addr)].try_into().unwrap(),
        }),
        sub_invocations: VecM::default(),
    }
}

fn entry(
    s: &Setup,
    delegates: std::vec::Vec<SorobanDelegateSignature>,
) -> SorobanAuthorizationEntry {
    SorobanAuthorizationEntry {
        credentials: SorobanCredentials::AddressWithDelegates(
            SorobanAddressCredentialsWithDelegates {
                address_credentials: SorobanAddressCredentials {
                    address: s.account.clone().into(),
                    nonce: 123,
                    signature_expiration_ledger: 100,
                    signature: auth_payload_scval(s),
                },
                delegates: delegates.try_into().unwrap(),
            },
        ),
        root_invocation: root_invocation(s),
    }
}

#[test]
fn delegated_signer_authenticates_via_cap71() {
    let s = setup();

    s.env.set_auths(&[entry(
        &s,
        std::vec![SorobanDelegateSignature {
            address: s.delegate.clone().into(),
            signature: ScVal::Void,
            nested_delegates: VecM::default(),
        }],
    )]);

    ProtectedClient::new(&s.env, &s.protected).protected(&s.account);
}

#[test]
fn delegated_signer_missing_from_credentials_is_rejected() {
    let s = setup();

    // The AuthPayload names the delegated signer, but the credentials carry no
    // delegate signature — `delegate_account_auth` must fail.
    s.env.set_auths(&[entry(&s, std::vec![])]);

    assert!(ProtectedClient::new(&s.env, &s.protected).try_protected(&s.account).is_err());
}

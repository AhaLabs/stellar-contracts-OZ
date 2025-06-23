
impl<T> FungibleToken for T
where
    T: Pausable + BaseToken,
{
    type Impl = Base;
    fn transfer(e: &Env, from: soroban_sdk::Address, to: soroban_sdk::Address, amount: i128) {
        Self::when_not_paused(e);
        Self::Impl::transfer(e, &from, &to, amount);
    }

    fn transfer_from(e: &Env, spender: soroban_sdk::Address, from: soroban_sdk::Address, to: soroban_sdk::Address, amount: i128) {
        Self::when_not_paused(e);
        Base::transfer_from(e, &spender, &from, &to, amount);
    }
}

impl<T> FungibleBurnable for T
where
    T: Pausable + BaseToken,
{
    fn burn(e: &Env, from: soroban_sdk::Address, amount: i128) {
        Self::when_not_paused(e);
        Base::burn(e, &from, amount);
    }
    fn burn_from(e: &Env, spender: soroban_sdk::Address, from: soroban_sdk::Address, amount: i128) {
        Self::when_not_paused(e);
        Base::burn_from(e, &spender, &from, amount);
    }
}

// Mintable

impl<T> FungibleMintable for T
where
    T: Administratable + Pausable + BaseToken,
{
    fn mint(e: &Env, to: soroban_sdk::Address, amount: i128) {
        Self::when_not_paused(e);
        Self::require_admin(e);
        Base::mint(e, &to, amount);
    }
}


impl<T> FungibleMintable for T
where
    T: Administratable + BaseToken,
{
    fn mint(e: &Env, to: soroban_sdk::Address, amount: i128) {
        Self::require_admin(e);
        Base::mint(e, &to, amount);
    }
}

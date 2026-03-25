#![no_std]

use soroban_sdk::{
    Address, Env, String, Symbol, contract, contracterror, contractevent, contractimpl,
    contracttype, panic_with_error, symbol_short,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum LRNError {
    Soulbound = 1,
    Unauthorized = 2,
    ZeroAmount = 3,
    NotInitialized = 4,
}

const ADMIN_KEY: Symbol = symbol_short!("ADMIN");
const NAME_KEY: Symbol = symbol_short!("NAME");
const SYMBOL_KEY: Symbol = symbol_short!("SYMBOL");
const DECIMALS_KEY: Symbol = symbol_short!("DECIMALS");

#[contracttype]
pub enum DataKey {
    Balance(Address),
    TotalSupply,
}

#[contractevent(topics = ["mint"])]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LRNMinted {
    #[topic]
    pub learner: Address,
    pub amount: i128,
}

#[contract]
pub struct LearnToken;

#[contractimpl]
impl LearnToken {
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&ADMIN_KEY) {
            panic_with_error!(&env, LRNError::NotInitialized);
        }
        admin.require_auth();

        env.storage().instance().set(&ADMIN_KEY, &admin);
        env.storage().instance().set(&NAME_KEY, &String::from_str(&env, "LearnToken"));
        env.storage().instance().set(&SYMBOL_KEY, &String::from_str(&env, "LRN"));
        env.storage().instance().set(&DECIMALS_KEY, &7u32);
        env.storage().instance().set(&DataKey::TotalSupply, &0i128);
    }

    pub fn mint(env: Env, to: Address, amount: i128) {
        let admin: Address = env.storage().instance().get(&ADMIN_KEY)
            .unwrap_or_else(|| panic_with_error!(&env, LRNError::NotInitialized));
        admin.require_auth();

        if amount <= 0 {
            panic_with_error!(&env, LRNError::ZeroAmount);
        }

        let balance_key = DataKey::Balance(to.clone());
        let current_balance: i128 = env.storage().persistent().get(&balance_key).unwrap_or(0);
        env.storage().persistent().set(&balance_key, &(current_balance + amount));

        let total_supply: i128 = env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0);
        env.storage().instance().set(&DataKey::TotalSupply, &(total_supply + amount));

        LRNMinted { learner: to, amount }.publish(&env);
    }

    pub fn set_admin(env: Env, new_admin: Address) {
        let admin: Address = env.storage().instance().get(&ADMIN_KEY)
            .unwrap_or_else(|| panic_with_error!(&env, LRNError::NotInitialized));
        admin.require_auth();
        env.storage().instance().set(&ADMIN_KEY, &new_admin);
    }

    pub fn balance(env: Env, account: Address) -> i128 {
        env.storage().persistent().get(&DataKey::Balance(account)).unwrap_or(0)
    }

    pub fn reputation_score(env: Env, account: Address) -> i128 {
        Self::balance(env, account)
    }

    pub fn total_supply(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0)
    }

    pub fn decimals(env: Env) -> u32 {
        env.storage().instance().get(&DECIMALS_KEY).unwrap_or(7)
    }

    pub fn name(env: Env) -> String {
        env.storage().instance().get(&NAME_KEY).unwrap()
    }

    pub fn symbol(env: Env) -> String {
        env.storage().instance().get(&SYMBOL_KEY).unwrap()
    }

    pub fn transfer(env: Env, _from: Address, _to: Address, _amount: i128) {
        panic_with_error!(&env, LRNError::Soulbound)
    }

    pub fn transfer_from(env: Env, _spender: Address, _from: Address, _to: Address, _amount: i128) {
        panic_with_error!(&env, LRNError::Soulbound)
    }

    pub fn approve(env: Env, _from: Address, _spender: Address, _amount: i128, _expiration: u32) {
        panic_with_error!(&env, LRNError::Soulbound)
    }

    pub fn allowance(_env: Env, _from: Address, _spender: Address) -> i128 {
        0
    }
}

#[cfg(test)]
mod test;
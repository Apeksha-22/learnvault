#![no_std]

use soroban_sdk::{
    Address, Env, String, Symbol, contract, contracterror, contractimpl, contracttype,
    panic_with_error, symbol_short,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    Unauthorized = 2,
    NotInitialized = 3,
    TokenNotFound = 4,
    Soulbound = 5,
    ScholarAlreadyMinted = 6,
}

#[derive(Clone)]
#[contracttype]
pub struct ScholarMetadata {
    pub scholar: Address,
    pub program_name: String,
    pub completion_date: u64,
    pub ipfs_uri: Option<String>,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    TokenCount,
    TokenOwner(u64),
    TokenUri(u64),
    ScholarToken(Address),
    Metadata(u64),
}

const ADMIN_KEY: Symbol = symbol_short!("ADMIN");
const TOKEN_COUNTER_KEY: Symbol = symbol_short!("CTR");

#[contract]
pub struct ScholarNft;

#[contractimpl]
impl ScholarNft {
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&ADMIN_KEY) {
            panic_with_error!(&env, Error::AlreadyInitialized);
        }

        admin.require_auth();
        env.storage().instance().set(&ADMIN_KEY, &admin);
        env.storage().instance().set(&TOKEN_COUNTER_KEY, &0_u64);
    }

    pub fn mint(env: Env, scholar: Address, program_name: String, ipfs_uri: Option<String>) -> u64 {
        let admin = Self::get_admin(&env);
        admin.require_auth();

        let scholar_key = DataKey::ScholarToken(scholar.clone());
        if env.storage().persistent().has(&scholar_key) {
            panic_with_error!(&env, Error::ScholarAlreadyMinted);
        }

        let next_token_id = Self::get_token_counter(&env).saturating_add(1);

        env.storage()
            .instance()
            .set(&TOKEN_COUNTER_KEY, &next_token_id);
        env.storage()
            .persistent()
            .set(&DataKey::TokenOwner(next_token_id), &scholar);
        env.storage().persistent().set(&scholar_key, &next_token_id);

        let metadata = ScholarMetadata {
            scholar: scholar.clone(),
            program_name,
            completion_date: env.ledger().timestamp(),
            ipfs_uri,
        };
        env.storage()
            .persistent()
            .set(&DataKey::Metadata(next_token_id), &metadata);

        next_token_id
    }

    pub fn get_token(env: Env, scholar: Address) -> Option<u64> {
        env.storage()
            .persistent()
            .get(&DataKey::ScholarToken(scholar))
    }

    pub fn get_metadata(env: Env, token_id: u64) -> Option<ScholarMetadata> {
        env.storage().persistent().get(&DataKey::Metadata(token_id))
    }

    pub fn has_credential(env: Env, scholar: Address) -> bool {
        env.storage()
            .persistent()
            .has(&DataKey::ScholarToken(scholar))
    }

    pub fn transfer(env: Env, _from: Address, _to: Address, _token_id: u64) {
        panic_with_error!(&env, Error::Soulbound);
    }

    // Helper functions
    fn get_token_counter(env: &Env) -> u64 {
        env.storage()
            .instance()
            .get(&TOKEN_COUNTER_KEY)
            .unwrap_or(0_u64)
    }

    fn get_admin(env: &Env) -> Address {
        env.storage()
            .instance()
            .get(&ADMIN_KEY)
            .unwrap_or_else(|| panic_with_error!(env, Error::NotInitialized))
    }
}

mod test;

extern crate std;

use soroban_sdk::{testutils::{Address as _, Events as _}, Address, Env, IntoVal, String};
use crate::{LRNError, LearnToken, LearnTokenClient};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn setup(e: &Env) -> (Address, Address, LearnTokenClient) {
    let admin = Address::generate(e);
    let id = e.register(LearnToken, ());
    e.mock_all_auths();
    let client = LearnTokenClient::new(e, &id);
    client.initialize(&admin);
    (id, admin, client)
}

// ---------------------------------------------------------------------------
// Initialization
// ---------------------------------------------------------------------------

#[test]
fn initialize_stores_metadata() {
    let e = Env::default();
    let (_, _, client) = setup(&e);
    assert_eq!(client.name(), String::from_str(&e, "LearnToken"));
    assert_eq!(client.symbol(), String::from_str(&e, "LRN"));
    assert_eq!(client.decimals(), 7);
}

#[test]
fn double_initialize_reverts() {
    let e = Env::default();
    let (_, admin, client) = setup(&e);
    // Hamaare lib.rs mein initialize NotInitialized panic karta hai agar ADMIN_KEY already ho
    let result = client.try_initialize(&admin);
    assert_eq!(
        result.err(),
        Some(Ok(soroban_sdk::Error::from_contract_error(
            LRNError::NotInitialized as u32
        )))
    );
}

// ---------------------------------------------------------------------------
// Minting
// ---------------------------------------------------------------------------

#[test]
fn mint_increases_balance_and_total_supply() {
    let e = Env::default();
    let (_, _, client) = setup(&e);
    let learner = Address::generate(&e);
    client.mint(&learner, &100); // Removed course_id as per lib.rs signature
    assert_eq!(client.balance(&learner), 100);
    assert_eq!(client.total_supply(), 100);
}

#[test]
fn mint_zero_amount_reverts() {
    let e = Env::default();
    let (_, _, client) = setup(&e);
    let learner = Address::generate(&e);
    let result = client.try_mint(&learner, &0);
    assert_eq!(
        result.err(),
        Some(Ok(soroban_sdk::Error::from_contract_error(
            LRNError::ZeroAmount as u32
        )))
    );
}

// ---------------------------------------------------------------------------
// Soulbound enforcement
// ---------------------------------------------------------------------------

#[test]
fn transfer_is_blocked() {
    let e = Env::default();
    let (_, _, client) = setup(&e);
    let a = Address::generate(&e);
    let b = Address::generate(&e);
    client.mint(&a, &50);
    let result = client.try_transfer(&a, &b, &10);
    assert_eq!(
        result.err(),
        Some(Ok(soroban_sdk::Error::from_contract_error(
            LRNError::Soulbound as u32
        )))
    );
}

// ---------------------------------------------------------------------------
// Access control
// ---------------------------------------------------------------------------

#[test]
fn set_admin_updates_admin() {
    let e = Env::default();
    let (_, _, client) = setup(&e);
    let new_admin = Address::generate(&e);
    client.set_admin(&new_admin);
    
    // Auth ko naye admin ke liye mock karna padega mint ke liye
    e.mock_all_auths(); 
    let learner = Address::generate(&e);
    client.mint(&learner, &10);
    assert_eq!(client.balance(&learner), 10);
}

// ---------------------------------------------------------------------------
// reputation_score
// ---------------------------------------------------------------------------

#[test]
fn reputation_score_mirrors_balance_after_mint() {
    let e = Env::default();
    let (_, _, client) = setup(&e);
    let learner = Address::generate(&e);
    client.mint(&learner, &200);
    assert_eq!(client.reputation_score(&learner), client.balance(&learner));
    assert_eq!(client.reputation_score(&learner), 200);
}

#[test]
fn mint_emits_event() {
    let e = Env::default();
    let (_, _, client) = setup(&e);
    let learner = Address::generate(&e);

    client.mint(&learner, &100);
    // Sirf itna check kar rahe hain ki event publish hua
    assert_eq!(e.events().all().len(), 1);
}
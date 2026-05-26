#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};
use soroban_sdk::token::{Client as TokenClient, StellarAssetClient};

fn setup_test() -> (Env, Address, Address, Address, TokenClient, StellarAssetClient, MedDisburseClient) {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let volunteer = Address::generate(&env);
    let unauthorized_user = Address::generate(&env);

    // Mock a Stellar Asset (e.g., USDC)
    let token_admin = Address::generate(&env);
    let token_contract = env.register_stellar_asset_contract(token_admin.clone());
    let token_client = TokenClient::new(&env, &token_contract);
    let token_admin_client = StellarAssetClient::new(&env, &token_contract);

    let contract_id = env.register_contract(None, MedDisburse);
    let client = MedDisburseClient::new(&env, &contract_id);

    (env, admin, volunteer, unauthorized_user, token_client, token_admin_client, client)
}

#[test]
fn test_1_happy_path_claim() {
    let (env, admin, volunteer, _, token_client, token_admin_client, client) = setup_test();
    let stipend_amount = 50_0000000; // 50 USDC

    client.initialize(&admin, &token_client.address, &stipend_amount);
    
    // Fund the treasury contract
    token_admin_client.mint(&client.address, &1000_0000000);

    // Whitelist volunteer and claim
    client.add_volunteer(&volunteer);
    client.claim_stipend(&volunteer);

    // Verify volunteer received funds
    assert_eq!(token_client.balance(&volunteer), stipend_amount);
    assert_eq!(token_client.balance(&client.address), 1000_0000000 - stipend_amount);
}

#[test]
#[should_panic(expected = "Caller is not an authorized volunteer")]
fn test_2_edge_case_unauthorized_claim() {
    let (env, admin, _, unauthorized_user, token_client, token_admin_client, client) = setup_test();
    let stipend_amount = 50_0000000;

    client.initialize(&admin, &token_client.address, &stipend_amount);
    token_admin_client.mint(&client.address, &1000_0000000);

    // Unauthorized user attempts to claim without being added to the whitelist
    client.claim_stipend(&unauthorized_user);
}

#[test]
fn test_3_state_verification_after_claim() {
    let (env, admin, volunteer, _, token_client, _, client) = setup_test();
    let stipend_amount = 50_0000000;

    client.initialize(&admin, &token_client.address, &stipend_amount);
    client.add_volunteer(&volunteer);
    
    // Check storage directly to ensure volunteer is marked true
    let is_vol: bool = env.as_contract(&client.address, || {
        env.storage().persistent().get(&DataKey::Volunteer(volunteer.clone())).unwrap()
    });
    assert!(is_vol);
}

#[test]
#[should_panic(expected = "Emergency stipend already claimed")]
fn test_4_edge_case_double_claim() {
    let (env, admin, volunteer, _, token_client, token_admin_client, client) = setup_test();
    let stipend_amount = 50_0000000;

    client.initialize(&admin, &token_client.address, &stipend_amount);
    token_admin_client.mint(&client.address, &1000_0000000);

    client.add_volunteer(&volunteer);
    client.claim_stipend(&volunteer);
    
    // Attempt to claim again
    client.claim_stipend(&volunteer);
}

#[test]
#[should_panic] // Panics due to require_auth failing on admin verification
fn test_5_edge_case_non_admin_adds_volunteer() {
    let (env, admin, volunteer, unauthorized_user, token_client, _, client) = setup_test();
    
    // Disable mock auth to test actual authorization bounds
    let env = Env::default(); 
    let contract_id = env.register_contract(None, MedDisburse);
    let strict_client = MedDisburseClient::new(&env, &contract_id);
    
    strict_client.initialize(&admin, &token_client.address, &50_0000000);

    // Attempting to add a volunteer with an unauthorized user signing the transaction
    strict_client.add_volunteer(&unauthorized_user); 
}
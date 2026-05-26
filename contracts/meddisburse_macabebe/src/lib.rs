#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env};

#[contracttype]
pub enum DataKey {
    Admin,
    Token,
    StipendAmount,
    Volunteer(Address),
    Claimed(Address),
}

#[contract]
pub struct MedDisburse;

#[contractimpl]
impl MedDisburse {
    /// Initializes the emergency treasury.
    /// Defines the LGU admin, the asset to distribute (e.g., USDC), and the fixed stipend amount.
    pub fn initialize(env: Env, admin: Address, token: Address, amount: i128) {
        admin.require_auth();
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Treasury already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::StipendAmount, &amount);
    }

    /// Admin adds a verified Barangay Health Worker (BHW) to the active whitelist.
    pub fn add_volunteer(env: Env, volunteer: Address) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        env.storage().persistent().set(&DataKey::Volunteer(volunteer), &true);
    }

    /// Volunteer claims their emergency stipend.
    /// Verifies whitelist status and prevents double-claiming.
    pub fn claim_stipend(env: Env, volunteer: Address) {
        volunteer.require_auth();
        
        // 1. Verify authorization
        let is_volunteer = env.storage().persistent().get::<_, bool>(&DataKey::Volunteer(volunteer.clone())).unwrap_or(false);
        if !is_volunteer {
            panic!("Caller is not an authorized volunteer");
        }
        
        // 2. Prevent double claims
        let has_claimed = env.storage().persistent().get::<_, bool>(&DataKey::Claimed(volunteer.clone())).unwrap_or(false);
        if has_claimed {
            panic!("Emergency stipend already claimed");
        }

        let token_id: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        let amount: i128 = env.storage().instance().get(&DataKey::StipendAmount).unwrap();
        
        // 3. Execute transfer from contract treasury to volunteer
        let token_client = token::Client::new(&env, &token_id);
        token_client.transfer(&env.current_contract_address(), &volunteer, &amount);

        // 4. Update state to reflect successful claim
        env.storage().persistent().set(&DataKey::Claimed(volunteer), &true);
    }
}
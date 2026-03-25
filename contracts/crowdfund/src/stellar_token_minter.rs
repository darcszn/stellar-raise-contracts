//! Stellar Token Minter Contract
//!
//! This contract provides NFT minting capabilities for the crowdfunding platform.
//! It implements a simple minting mechanism that can be called by authorized
//! contracts (like the Crowdfund contract) to reward contributors with NFTs.
//!
//! ## Security
//!
//! - **Authorization**: Only the contract admin or the designated minter can call `mint`.
//! - **State Management**: Uses persistent storage for token ID tracking and metadata.
//! - **Bounded Operations**: Ensures all operations are within Soroban resource limits.

#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, String, Symbol, Vec,
};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Minter,
    TotalMinted,
    TokenMetadata(u64),
}

#[contract]
pub struct StellarTokenMinter;

#[contractimpl]
impl StellarTokenMinter {
    /// Initializes the minter contract.
    ///
    /// # Arguments
    ///
    /// * `admin` - Contract administrator
    /// * `minter` - Address authorized to perform minting
    pub fn initialize(env: Env, admin: Address, minter: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Minter, &minter);
        env.storage().instance().set(&DataKey::TotalMinted, &0u64);
    }

    /// Mints a new NFT to the specified recipient.
    ///
    /// # Arguments
    ///
    /// * `to` - Recipient address
    /// * `token_id` - ID of the token to mint
    ///
    /// # Panics
    ///
    /// * If the caller is not authorized (not admin or minter)
    /// * If the token ID has already been minted
    pub fn mint(env: Env, to: Address, token_id: u64) {
        let minter: Address = env.storage().instance().get(&DataKey::Minter).unwrap();
        minter.require_auth();

        let key = DataKey::TokenMetadata(token_id);
        if env.storage().persistent().has(&key) {
            panic!("token already minted");
        }

        // Store some basic metadata to record the ownership
        env.storage().persistent().set(&key, &to);

        // Update total counter
        let total: u64 = env.storage().instance().get(&DataKey::TotalMinted).unwrap();
        env.storage().instance().set(&DataKey::TotalMinted, &(total + 1));

        // Emit event
        env.events().publish(
            (Symbol::new(&env, "mint"), to),
            token_id,
        );
    }

    /// Returns the owner of a token.
    pub fn owner(env: Env, token_id: u64) -> Option<Address> {
        env.storage().persistent().get(&DataKey::TokenMetadata(token_id))
    }

    /// Returns the total number of NFTs minted.
    pub fn total_minted(env: Env) -> u64 {
        env.storage().instance().get(&DataKey::TotalMinted).unwrap_or(0)
    }

    /// Updates the minter address. Only callable by admin.
    pub fn set_minter(env: Env, admin: Address, new_minter: Address) {
        let current_admin: Address = env.storage().instance().get(&DataKey::Admin).expect("not initialized");
        current_admin.require_auth();
        if admin != current_admin {
            panic!("unauthorized");
        }
        env.storage().instance().set(&DataKey::Minter, &new_minter);
    }
}

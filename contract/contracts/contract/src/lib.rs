#![no_std]

use soroban_sdk::{
    contract, contractimpl, symbol_short, Env, Symbol, Map, Address
};

#[contract]
pub struct PredictionDashboard;

#[contractimpl]
impl PredictionDashboard {

    // Create a new prediction
    pub fn create_prediction(env: Env, id: Symbol) {
        let votes: Map<Symbol, u32> = Map::new(&env);
        
        let mut votes = votes;
        votes.set(symbol_short!("yes"), 0);
        votes.set(symbol_short!("no"), 0);

        env.storage().instance().set(&id, &votes);
    }

    // Vote (with authentication)
    pub fn vote(env: Env, user: Address, id: Symbol, option: Symbol) {
        // Require signature
        user.require_auth();

        let mut votes: Map<Symbol, u32> =
            env.storage().instance().get(&id)
            .expect("Prediction not found");

        let current = votes.get(option.clone()).unwrap_or(0);
        votes.set(option, current + 1);

        env.storage().instance().set(&id, &votes);
    }

    // Get results safely
    pub fn get_results(env: Env, id: Symbol) -> Map<Symbol, u32> {
        env.storage().instance().get(&id)
            .expect("Prediction not found")
    }
}
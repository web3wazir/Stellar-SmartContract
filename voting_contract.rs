#![no_std]

use soroban_sdk::{contractimpl, symbol_short, Address, Env, Symbol, Vec, Map};

pub struct VotingContract;

#[contractimpl]
impl VotingContract {
    pub fn initialize(env: Env) {
        env.storage().set(symbol_short!("candidates"), &Vec::<Symbol>::new(&env));
        env.storage().set(symbol_short!("votes"), &Map::<Symbol, u32>::new(&env));
        env.storage().set(symbol_short!("voted"), &Map::<Address, bool>::new(&env));
    }
    pub fn add_candidate(env: Env, candidate: Symbol) {
        let mut candidates: Vec<Symbol> = env.storage().get_unchecked(symbol_short!("candidates")).unwrap();
        if candidates.contains(&candidate) {
            panic!("Candidate already exists");
        }
        candidates.push_back(candidate.clone());
        env.storage().set(symbol_short!("candidates"), &candidates);

        let mut votes: Map<Symbol, u32> = env.storage().get_unchecked(symbol_short!("votes")).unwrap();
        votes.set(candidate, 0);
        env.storage().set(symbol_short!("votes"), &votes);
    }
    pub fn vote(env: Env, voter: Address, candidate: Symbol) {
        voter.require_auth(); // Require voter to sign transaction

        let mut voted: Map<Address, bool> = env.storage().get_unchecked(symbol_short!("voted")).unwrap();
        if voted.get(voter.clone()).unwrap_or(false) {
            panic!("Already voted");
        }

        let mut votes: Map<Symbol, u32> = env.storage().get_unchecked(symbol_short!("votes")).unwrap();
        if !votes.contains_key(candidate.clone()) {
            panic!("Invalid candidate");
        }

        let current = votes.get(candidate.clone()).unwrap();
        votes.set(candidate.clone(), current + 1);
        env.storage().set(symbol_short!("votes"), &votes);

        voted.set(voter, true);
        env.storage().set(symbol_short!("voted"), &voted);
    }
    pub fn get_votes(env: Env, candidate: Symbol) -> u32 {
        let votes: Map<Symbol, u32> = env.storage().get_unchecked(symbol_short!("votes")).unwrap();
        votes.get(candidate).unwrap_or(0)
    }
    pub fn get_candidates(env: Env) -> Vec<Symbol> {
        env.storage().get_unchecked(symbol_short!("candidates")).unwrap()
    }
}

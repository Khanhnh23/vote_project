#![cfg(test)]

use super::*;
use soroban_sdk::{Env, Address, symbol_short};

#[test]
fn test_voting() {
    let env = Env::default();

    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    // 👤 fake users
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let user3 = Address::generate(&env);

    // 🗳️ vote KH
    client.vote_khanh(&user1);
    client.vote_khanh(&user2);

    // 🗳️ vote TH
    client.vote_thu(&user3);

    // 📊 check result
    let result = client.get_results();

    assert_eq!(result.0, 2);
    assert_eq!(result.1, 1);

    // 💰 fee check
    let fee = client.get_total_fee();
    assert_eq!(fee, 3);

    // 🏆 winner
    let winner = client.get_winner();
    assert_eq!(winner, symbol_short!("KH"));
}
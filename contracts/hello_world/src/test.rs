#[cfg(test)]
use super::*;
use soroban_sdk::{ symbol_short, vec, Env };

#[test]
pub fn add_message() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TossContract);
    let client = TossContractClient::new(&env, &contract_id);

    let result = client.add_message(
        &String::from_str(&env, "asdfk"),
        &String::from_str(&env, "asd;fjkd")
    );

    assert_eq!(
        result,
        vec![&env, Message {
            content: String::from_str(&env, "asdfk"),
            address: String::from_str(&env, "asd;fjkd"),
        }]
    )
}
#[test]
pub fn get_message() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TossContract);
    let client = TossContractClient::new(&env, &contract_id);

    let result = client.get_message();

    assert_eq!(
        result,
        vec![&env, Message {
            content: String::from_str(&env, "asdfk"),
            address: String::from_str(&env, "asd;fjkd"),
        }]
    )
}

#![no_std]
use soroban_sdk::{ contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec };

#[contracttype]
pub struct MessagesStruct {
    pub messages: Vec<String>,
}

pub const STATE: Symbol = symbol_short!("STATE");

#[contract]
pub struct TossContract;

#[contractimpl]
impl TossContract {
    pub fn toss(env: Env) -> Symbol {
        let res: u64 = env.prng().gen_range(1..=100);
        if res % 2 == 0 {
            symbol_short!("Tails")
        } else {
            symbol_short!("Heads")
        }
    }

    // pub fn change_state(env: Env) {
    //     env.storage().instance().set(&STATE, &symbol_short!("lol"))
    // }

    // pub fn get_state(env: Env) -> Symbol {
    //     env.storage().instance().get(&STATE).unwrap_or(symbol_short!("Err"))
    // }

    pub fn get_message(env: Env) -> Vec<String> {
        env
            .storage()
            .instance()
            .get(&STATE)
            .unwrap_or(MessagesStruct { messages: Vec::new(&env) }).messages
    }
    pub fn add_message(env: Env, message: String) -> Vec<String> {
        let mut all_messages = env
            .storage()
            .instance()
            .get(&STATE)
            .unwrap_or(MessagesStruct { messages: Vec::new(&env) }).messages;

        all_messages.push_back(message);

        env.storage().instance().set(&STATE, &(MessagesStruct { messages: all_messages.clone() }));

        all_messages
    }
}

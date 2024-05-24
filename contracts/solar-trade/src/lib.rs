#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, token, Address, Env, Symbol};


const ID: Symbol = symbol_short!("ID");


#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Trade,
}


#[derive(Debug,Clone, PartialEq)]
#[contracttype]
pub struct Trade {
    id: u32,
    seller: Address,
    energy_amount: u32,
    price: u32,
    withdraw_amount: i128,
}

// #[derive(Clone)]
// #[contracttype]
// pub struct Trades {
//     trades : Vec<Trade>
// }


#[contract]
pub struct SolarTrade;

#[contractimpl]
impl SolarTrade {
    pub fn create(env: Env, seller: Address, energy_amount: u32, price: u32) {
        seller.require_auth();

        let mut id: u32 = env.storage().instance().get(&ID).unwrap_or(0);

        id += 1;

        write_trade(&env, &Trade {
            id,
            seller,
            energy_amount,
            price,
            withdraw_amount: 0,
        });
        
        update_id(&env, id);
    }

    pub fn buy_energy(env: Env, buyer: Address, trade_id: u32, energy_amount: u32) {
        buyer.require_auth();

        let mut trade: Trade = get_trade(&env, trade_id);

        if trade.energy_amount < energy_amount {
            panic!("Not enough energy");
        }

        let amount: i128 = (trade.price * energy_amount) as i128;

        let buyer_token_client = token::Client::new(&env, &buyer);

        let contract = env.current_contract_address();

        buyer_token_client.transfer(&buyer, &contract, &amount);

        trade.energy_amount -= energy_amount;

        trade.withdraw_amount += amount;  

        env.storage().persistent().set(&trade_id, &trade);

    }

    pub fn withdraw_amount(env: Env, seller: Address, trade_id: u32) {
        seller.require_auth();

        let mut trade: Trade = get_trade(&env, trade_id);

        if seller != trade.seller {
            panic!("Anuthorized")
        }

        let seller_token_client = token::Client::new(&env, &seller);

        let contract = env.current_contract_address();

        seller_token_client.transfer(&contract, &seller, &trade.withdraw_amount);

        trade.withdraw_amount = 0;

        env.storage().persistent().set(&trade_id, &trade);

    }

}

fn write_trade(env: &Env, trade: &Trade) {
    env.storage().persistent().set(&trade.id, trade);
}

fn get_trade(env: &Env, trade_id: u32) -> Trade {
    env.storage().persistent().get(&trade_id).unwrap()
}

fn update_id(env: &Env, id: u32) {
    env.storage().instance().set(&ID, &id);
    env.storage().instance().extend_ttl(100, 100);
}
mod test;

// CC6KLR6JR7RTZUHN7F3YESTVGO4EUPUN6YJAXQ5JTSS2Q3EOXS3PEKXW

soroban contract invoke \                               
  --id CC6KLR6JR7RTZUHN7F3YESTVGO4EUPUN6YJAXQ5JTSS2Q3EOXS3PEKXW \
  --source alice \
  --network testnet \
  -- \
  create \
  --seller GA2U2XAPTNGKRK74HGBDCBZK7HJASP6LFK2DHPQZECMDSYDE6U5CTWTB \
  --energy_amount 100 \
  --price 10
#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, contracterror, symbol_short, token, Address, Env, Symbol, Vec};

const ID: Symbol = symbol_short!("ID");

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NoData = 1,
    NotValidTradeId = 2,
    Unauthorized = 3,
    UnsufficentEnergy = 4
}

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
            withdraw_amount: 0_i128,
        }); 
        
        update_id(&env, id);
    }

    pub fn buy_energy(env: Env, buyer: Address, trade_id: u32, energy_amount: u32) -> Result<Trade, Error>  {
        buyer.require_auth();

        let mut trade: Trade = get_trade(&env, trade_id)?;

        if trade.energy_amount < energy_amount {
            return Err(Error::UnsufficentEnergy)
        }

        let amount: i128 = (trade.price * energy_amount) as i128;

        let buyer_token_client = token::Client::new(&env, &buyer);

        let contract = env.current_contract_address();

        buyer_token_client.transfer(&buyer, &contract, &amount);

        trade.energy_amount -= energy_amount;

        trade.withdraw_amount += amount;  

        env.storage().persistent().set(&trade_id, &trade);

        Ok(trade)

    }

    pub fn withdraw_amount(env: Env, seller: Address, trade_id: u32) -> Result<Trade, Error> {
        seller.require_auth();

        let mut trade: Trade = get_trade(&env, trade_id)?;

        if seller != trade.seller {
            return Err(Error::Unauthorized)
        }

        let seller_token_client = token::Client::new(&env, &seller);

        let contract = env.current_contract_address();

        seller_token_client.transfer(&contract, &seller, &trade.withdraw_amount);

        trade.withdraw_amount = 0;

        env.storage().persistent().set(&trade_id, &trade);

        Ok(trade)

    }

    pub fn get_trades(env: Env) -> Result<Vec<Trade>, Error>{
        let mut trades: Vec<Trade> = Vec::new(&env);
        let id = match env.storage().instance().get(&ID) {
            Some(id) => id,
            None => return Err(Error::NoData)
        };

        for trade_id in 1..=id {
            let trade: Trade = get_trade(&env, trade_id)?;
            trades.push_back(trade);
        }

        Ok(trades)
    }

}

fn write_trade(env: &Env, trade: &Trade) {
    env.storage().persistent().set(&trade.id, trade);
}

fn get_trade(env: &Env, trade_id: u32) -> Result<Trade, Error> {
    match env.storage().persistent().get(&trade_id) {
        Some(trade) => Ok(trade),
        None => return Err(Error::NotValidTradeId)
    }
}

fn update_id(env: &Env, id: u32) {
    env.storage().instance().set(&ID, &id);
    env.storage().instance().extend_ttl(100, 100);
}   
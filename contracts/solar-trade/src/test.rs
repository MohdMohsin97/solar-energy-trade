#![cfg(test)]

use super::*;
use soroban_sdk::{ testutils::Address as _, Address, Env};

// fn create_token_contract<'a>(
//     e: &Env,
//     admin: &Address,
// ) -> (token::Client<'a>, token::StellarAssetClient<'a>) {
//     let addr = e.register_stellar_asset_contract(admin.clone());
//     (
//         token::Client::new(e, &addr),
//         token::StellarAssetClient::new(e, &addr),
//     )
// }

#[test]
fn test() {
    let env = Env::default();
    env.mock_all_auths();
    let seller = Address::generate(&env);
    let contract_id = env.register_contract(None, SolarTrade);
    let client = SolarTradeClient::new(&env, &contract_id);
    assert_eq!(client.create(&seller, &100_u32, &10_u32), 1);
    assert_eq!(client.create(&seller, &100_u32, &10_u32), 2);
    assert_eq!(client.create(&seller, &100_u32, &10_u32), 3);

}

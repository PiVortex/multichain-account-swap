use near_sdk::{near, AccountId};
use near_sdk::collections::{UnorderedMap};

#[near(contract_state)]
pub struct Contract {
    trades: UnorderedMap<String, Trade>,
    trade_count: u32
}

#[near(serializers = [borsh, json])]
pub struct Trade {
    seller_id: AccountId,
    buyer_id: Option<AccountId>,
    eth_add: String,
    btc_add: String,
    nft_contract: String,
    nft_id: u32,
    sale_price: u32,
    swapped: Bool
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            trades: UnorderedMap::new(b"t"),
            trade_count: 0
        }
    }
}

#[near]
impl Contract {

    pub fn create_trade(&mut self, nft_contract: String, sale_price: u32) -> u32 {

    }

    pub fn accept_trade(&mut self) {

    }

    pub fn confirm_trade(&mut self) {

    }

    pub fn withdraw_nft(&mut self, payload: String) {

    }

    pub fn withdraw_btc(&mut self, paylod: String) {

    }
    
    pub fn get_all_trades(&self) -> Vec<String, Trade> {

    }

    pub fn get_trade(&self) -> Trade {

    }
}

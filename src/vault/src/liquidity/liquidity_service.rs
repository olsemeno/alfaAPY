use async_trait::async_trait;
use candid::Nat;
use kongswap_canister::add_liquidity::AddLiquidityReply;


enum DeFiProvider {
    Kong,
}

enum UserToken {
    ICP,
    ckBTC
}

#[async_trait]
pub trait  TokenVault {
    fn getToken(&self) -> UserToken;
    fn invest(&self, amount: Nat) -> Result<AddLiquidityReply, String>;
    fn withdraw(&self, amount: Nat) -> Result<Nat, String>;
    fn rebalance(&self) -> Result<(), String>;
    fn get_balance(&self) -> Result<(), String>;
}



#[async_trait]
pub trait DeFiPoolsProvider {
    async fn get_pools(&self) -> Result<DefiPoolsReply, String>;
    async fn add_liquidity(&self, token_0: String, amount_0: Nat, token_1: String, amount_1: Nat) -> Result<AddLiquidityResponse, String>;
    async fn remove_liquidity(&self, token_0: String, token_1: String, remove_lp_token_amount: Nat) -> Result<(), String>;
}



#[async_trait]
pub trait RequestRepo {
    async fn store_request(&self, request: ProviderRequest) -> Result<(), String>;
}


pub struct AddLiquidityResponse {
    pub request_id: u64,
    pub token_0: String,
    pub amount_0: Nat,
    pub token_1: String,
    pub amount_1: Nat,
    pub add_lp_token_amount: Nat,
}


pub struct ProviderRequest {
    pub request_id: u64,
    pub de_fi_provider: DeFiProvider
}



pub struct  DefiPoolsReply {
    pub pools: Vec<DefiPool>,
}

pub struct DefiPool {
    pub token_0: String,
    pub token_1: String,
    pub apy: Nat,
}
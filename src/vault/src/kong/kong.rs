use crate::token_swaps::swap_service::KONG_BE_CANISTER;
use ic_cdk::trap;
use kongswap_canister::pools::Response;

pub async fn pools() -> Response {
    kongswap_canister_c2c_client::pools(KONG_BE_CANISTER).await.unwrap_or_else(|(code, msg)| {
        trap(format!(
            "An error happened during the pools call: {}: {}",
            code as u8, msg
        ).as_str())
    })
}
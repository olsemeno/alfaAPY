use ic_cdk::api::management_canister::main::CanisterId;
use ic_cdk::trap;
use kong_swap_canister::pools::Response;
const KONG_BE_CANISTER: CanisterId = CanisterId::from_slice(&[0, 0, 0, 0, 2, 48, 2, 23, 1, 1]);


pub async fn pools() -> Response {
    kong_swap_c2c_client::pools(KONG_BE_CANISTER).await.unwrap_or_else(|(code, msg)| {
        trap(format!(
            "An error happened during the pools call: {}: {}",
            code as u8, msg
        ).as_str())
    })
}
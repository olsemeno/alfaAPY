use once_cell::sync::Lazy;
use types::CanisterId;

use crate::util::principal_to_canister_id;

// ================= PRINCIPALS =================

// POOL STATS PRINCIPAL
pub const POOL_STATS_PRINCIPAL: &str = "oxawg-7aaaa-aaaag-aub6q-cai";

// TOKEN PRINCIPALS
pub const ICP_TOKEN_PRINCIPAL: &str = "ryjl3-tyaaa-aaaaa-aaaba-cai";
pub const CKUSDT_TOKEN_PRINCIPAL: &str = "cngnf-vqaaa-aaaar-qag4q-cai";
pub const CKBTC_TOKEN_PRINCIPAL: &str = "mxzaz-hqaaa-aaaar-qaada-cai";
pub const PANDA_TOKEN_PRINCIPAL: &str = "druyg-tyaaa-aaaaq-aactq-cai";
pub const NFIDW_TOKEN_PRINCIPAL: &str = "mih44-vaaaa-aaaaq-aaekq-cai";
pub const ICS_TOKEN_PRINCIPAL: &str = "ca6gz-lqaaa-aaaaq-aacwa-cai";

// ICPSWAP PRINCIPALS
pub const ICPSWAP_SWAP_FACTORY_PRINCIPAL: &str = "4mmnk-kiaaa-aaaag-qbllq-cai";
pub const ICPSWAP_SWAP_CALCULATOR_PRINCIPAL: &str = "phr2m-oyaaa-aaaag-qjuoq-cai";
pub const ICPSWAP_NODE_INDEX_PRINCIPAL: &str = "ggzvv-5qaaa-aaaag-qck7a-cai";
pub const ICPSWAP_GLOBAL_INDEX_PRINCIPAL: &str = "gp26j-lyaaa-aaaag-qck6q-cai";

// KONGSWAP PRINCIPALS
pub const KONGSWAP_CANISTER_PRINCIPAL: &str = "2ipq2-uqaaa-aaaar-qailq-cai";

// ================= CANISTER IDS =================

// POOL STATS CANISTER ID
pub static POOL_STATS_CANISTER_ID: Lazy<CanisterId> =
    Lazy::new(|| principal_to_canister_id(POOL_STATS_PRINCIPAL));

// TOKEN CANISTER IDS
pub static ICP_TOKEN_CANISTER_ID: Lazy<CanisterId> =
    Lazy::new(|| principal_to_canister_id(ICP_TOKEN_PRINCIPAL));
pub static CKUSDT_TOKEN_CANISTER_ID: Lazy<CanisterId> =
    Lazy::new(|| principal_to_canister_id(CKUSDT_TOKEN_PRINCIPAL));
pub static CKBTC_TOKEN_CANISTER_ID: Lazy<CanisterId> =
    Lazy::new(|| principal_to_canister_id(CKBTC_TOKEN_PRINCIPAL));
pub static PANDA_TOKEN_CANISTER_ID: Lazy<CanisterId> =
    Lazy::new(|| principal_to_canister_id(PANDA_TOKEN_PRINCIPAL));
pub static NFIDW_TOKEN_CANISTER_ID: Lazy<CanisterId> =
    Lazy::new(|| principal_to_canister_id(NFIDW_TOKEN_PRINCIPAL));
pub static ICS_TOKEN_CANISTER_ID: Lazy<CanisterId> =
    Lazy::new(|| principal_to_canister_id(ICS_TOKEN_PRINCIPAL));


// ICPSWAP CANISTER IDS
pub static ICPSWAP_SWAP_FACTORY_CANISTER_ID: Lazy<CanisterId> =
    Lazy::new(|| principal_to_canister_id(ICPSWAP_SWAP_FACTORY_PRINCIPAL));
pub static ICPSWAP_SWAP_CALCULATOR_CANISTER_ID: Lazy<CanisterId> =
    Lazy::new(|| principal_to_canister_id(ICPSWAP_SWAP_CALCULATOR_PRINCIPAL));
pub static ICPSWAP_NODE_INDEX_CANISTER_ID: Lazy<CanisterId> =
    Lazy::new(|| principal_to_canister_id(ICPSWAP_NODE_INDEX_PRINCIPAL));
pub static ICPSWAP_GLOBAL_INDEX_CANISTER_ID: Lazy<CanisterId> =
    Lazy::new(|| principal_to_canister_id(ICPSWAP_GLOBAL_INDEX_PRINCIPAL));

// KONGSWAP CANISTER IDS
pub static KONGSWAP_CANISTER_ID: Lazy<CanisterId> =
    Lazy::new(|| principal_to_canister_id(KONGSWAP_CANISTER_PRINCIPAL));

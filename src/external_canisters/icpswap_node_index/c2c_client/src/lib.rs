use icpswap_node_index_canister::*;
use canister_client::{generate_candid_c2c_call_no_args};

// Queries
generate_candid_c2c_call_no_args!(getAllTokens);
generate_candid_c2c_call_no_args!(tvlStorageCanister);

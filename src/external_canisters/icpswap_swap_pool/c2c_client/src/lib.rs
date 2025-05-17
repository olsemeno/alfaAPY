use canister_client::{generate_candid_c2c_call, generate_candid_c2c_call_no_args};
use icpswap_swap_pool_canister::*;

// Queries
generate_candid_c2c_call!(quote);

// Updates
generate_candid_c2c_call!(depositFrom);
generate_candid_c2c_call!(swap);
generate_candid_c2c_call!(withdraw);
generate_candid_c2c_call_no_args!(getTokenMeta);

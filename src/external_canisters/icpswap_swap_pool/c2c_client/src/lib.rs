use canister_client::{generate_candid_c2c_call, generate_candid_c2c_call_no_args};
use icpswap_swap_pool_canister::*;

// Queries
generate_candid_c2c_call!(quote);
generate_candid_c2c_call_no_args!(metadata);
generate_candid_c2c_call!(getUserPositionIdsByPrincipal);
generate_candid_c2c_call!(getUserPosition);

// Updates
generate_candid_c2c_call!(depositFrom);
generate_candid_c2c_call!(swap);
generate_candid_c2c_call!(withdraw);
generate_candid_c2c_call!(increaseLiquidity);
generate_candid_c2c_call!(decreaseLiquidity);
generate_candid_c2c_call!(mint);
generate_candid_c2c_call!(claim);
generate_candid_c2c_call_no_args!(getTokenMeta);
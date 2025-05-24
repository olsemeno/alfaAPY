use canister_client::{generate_candid_c2c_call, generate_candid_c2c_call_no_args, generate_candid_c2c_call_tuple_args};
use icpswap_swap_pool_canister::*;

// Queries
generate_candid_c2c_call!(quote);
generate_candid_c2c_call!(getUserUnusedBalance);
generate_candid_c2c_call_tuple_args!(getUserPosition);
generate_candid_c2c_call_tuple_args!(getUserPositionIdsByPrincipal);
generate_candid_c2c_call_tuple_args!(getUserPositionsByPrincipal);
generate_candid_c2c_call_no_args!(metadata);

// Updates
generate_candid_c2c_call!(depositFrom);
generate_candid_c2c_call!(swap);
generate_candid_c2c_call!(withdraw);
generate_candid_c2c_call!(increaseLiquidity);
generate_candid_c2c_call!(decreaseLiquidity);
generate_candid_c2c_call!(mint);
generate_candid_c2c_call!(claim);
generate_candid_c2c_call_no_args!(getTokenMeta);

use icpswap_swap_calculator_canister::*;
use canister_client::{generate_candid_c2c_call_tuple_args};

// Queries
generate_candid_c2c_call_tuple_args!(getPrice);

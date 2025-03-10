use canister_client::{generate_candid_c2c_call, generate_candid_c2c_call_no_args};
pub use kongswap_canister::*; //TODO visibility

// Queries
generate_candid_c2c_call_no_args!(pools);
generate_candid_c2c_call!(swap_amounts);
generate_candid_c2c_call!(user_balances);
generate_candid_c2c_call!(requests);
generate_candid_c2c_call!(remove_liquidity_amounts);
generate_candid_c2c_call!(add_liquidity_amounts);

// Updates
generate_candid_c2c_call!(swap);
generate_candid_c2c_call!(add_liquidity);
generate_candid_c2c_call!(remove_liquidity);

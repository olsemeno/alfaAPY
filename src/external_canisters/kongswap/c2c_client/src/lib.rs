use canister_client::{generate_candid_c2c_call, generate_candid_c2c_call_no_args};
pub use kongswap_canister::*; //TODO visibility

// Queries

// Updates
generate_candid_c2c_call!(swap);
generate_candid_c2c_call_no_args!(pools);

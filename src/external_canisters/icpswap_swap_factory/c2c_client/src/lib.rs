use icpswap_swap_factory_canister::*;
use canister_client::generate_candid_c2c_call;

// Queries
generate_candid_c2c_call!(get_pool);
generate_candid_c2c_call!(get_pools);

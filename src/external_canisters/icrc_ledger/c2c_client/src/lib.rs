use canister_client::{generate_candid_c2c_call, generate_candid_c2c_call_no_args};
use icrc_ledger_canister::*;

// Queries
generate_candid_c2c_call!(icrc1_balance_of);
generate_candid_c2c_call_no_args!(icrc1_decimals);
generate_candid_c2c_call_no_args!(icrc1_fee);
generate_candid_c2c_call_no_args!(icrc1_metadata);
generate_candid_c2c_call_no_args!(icrc1_name);
generate_candid_c2c_call_no_args!(icrc1_supported_standards);
generate_candid_c2c_call_no_args!(icrc1_symbol);
generate_candid_c2c_call_no_args!(icrc1_total_supply);

// Updates
generate_candid_c2c_call!(icrc2_approve);
generate_candid_c2c_call!(icrc2_transfer_from);


// pub async fn icrc1_transfer(
//     canister_id: ::types::CanisterId,
//     args: &icrc1_transfer::Args,
// ) -> ::ic_cdk::api::call::CallResult<icrc1_transfer::Response> {
//     let method_name = "icrc1_transfer";
//     let args = args.clone();
//     canister_client::make_c2c_call(canister_id, method_name, &args, ::candid::encode_one, |r| {
//         ::candid::decode_one(r)
//     })
//     .await
// }

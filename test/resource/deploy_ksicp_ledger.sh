#!/usr/bin/env bash


TOKEN_SYMBOL="USDT"
TOKEN_LEDGER=$(echo ${TOKEN_SYMBOL}_ledger | tr '[:upper:]' '[:lower:]')
TOKEN_NAME="USD Tether (Test Token)"
TOKEN_DECIMALS=6
TRANSFER_FEE=10_000
TOKEN_LOGO="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADAAAAAwCAYAAABXAvmHAAAAAXNSR0IArs4c"

NETWORK="--network $1"

TRIGGER_THRESHOLD=10_000
NUM_OF_BLOCK_TO_ARCHIVE=5_000
CYCLE_FOR_ARCHIVE_CREATION=5_000_000_000_000
ICRC2_FEATURE_FLAG=true

dfx deploy ledger --specified-id=ryjl3-tyaaa-aaaaa-aaaba-cai --mode reinstall  -y --argument "(variant {Init =
	record {
		token_symbol = \"${TOKEN_SYMBOL}\";
		token_name = \"${TOKEN_NAME}\";
		decimals = opt ${TOKEN_DECIMALS};
		minting_account = record { owner = principal \"${CONTROLLER_PRINCIPAL_ID}\" };
		transfer_fee = ${TRANSFER_FEE};
		initial_balances = vec {};
		metadata = vec { record { \"icrc1:logo\"; variant { Text = \"${TOKEN_LOGO}\" }; }; };
		feature_flags = opt record { icrc2 = ${ICRC2_FEATURE_FLAG} };
		archive_options = record {
			num_blocks_to_archive = ${NUM_OF_BLOCK_TO_ARCHIVE};
			trigger_threshold = ${TRIGGER_THRESHOLD};
			controller_id = principal \"${CONTROLLER_PRINCIPAL_ID}\";
			cycles_for_archive_creation = opt ${CYCLE_FOR_ARCHIVE_CREATION};
		};
	}
})"

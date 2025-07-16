#!/bin/bash

echo "===== Running VAULT canister tests ====="
cargo test -q -p vault --lib -- --test-threads=1 2>/dev/null
vault_result=$?

echo -e "\n===== Running POOL_STATS canister tests ====="
cargo test -q -p pool_stats --lib -- --test-threads=1 2>/dev/null
pool_stats_result=$?

# Exit with failure if any test failed
if [ $pool_stats_result -ne 0 ] || [ $vault_result -ne 0 ]; then
    exit 1
fi

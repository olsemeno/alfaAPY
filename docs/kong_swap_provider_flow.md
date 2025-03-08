# Working with Pools

Canister - [Dashboard Link](https://dashboard.internetcomputer.org/canister/2ipq2-uqaaa-aaaar-qailq-cai)

### Pools We Work With

- `ckBTC_ckUSDT` (for ckBTC)
- `ckBTC_ICP` (for ckBTC, ICP)
- `ICP_ckUSDT` (for ICP)
- `ckETH_ICP` (for ICP)

## 1. Retrieve All Pools and Select the Required One

Method - `pools`.

From the list, select the required pool by its name (field `lp_token_symbol`).

Example: Selecting the pool `ckBTC_ckUSDT`.

```json
{
    "tvl": 472308535008,
    "lp_token_symbol": "ckBTC_ckUSDT",
    "name": "ckBTC_ckUSDT Liquidity Pool",
    "lp_fee_0": 687956,
    "lp_fee_1": 623587902,
    "balance_0": 272927992,
    "balance_1": 235540785270,
    "rolling_24h_volume": 455650938092,
    "rolling_24h_apy": 79.64,
    "address_0": "mxzaz-hqaaa-aaaar-qaada-cai",
    "address_1": "cngnf-vqaaa-aaaar-qag4q-cai",
    "rolling_24h_num_swaps": 3792,
    "symbol_0": "ckBTC",
    "symbol_1": "ckUSDT",
    "pool_id": 2,
    "price": 86312.36,
    "chain_0": "IC",
    "chain_1": "IC",
    "is_removed": false,
    "symbol": "ckBTC_ckUSDT",
    "rolling_24h_lp_fee": 1030502982,
    "lp_fee_bps": 30
}
```

### APY, APR Calculation

```rust
    let daily_yield = rolling_24h_lp_fee / tvl;
    let apr = daily_yield * 365.0 * 100;
    let apy = (1.0 + daily_yield).powf(365.0) - 1.0;
```

**Update:** Actually, for this DEX, no manual calculation is needed since the field `rolling_24h_apy` already provides the required value. However, it is not a true APY but an APR (as confirmed by the formula above). The value represents the past 24 hours' APY and will fluctuate daily. For now, we use this field, but in the future, we can store daily data and compute an average over a selected period.

The formulas will still be useful for other DEXs.

---

## 2. Compute Token Ratio for Adding Liquidity

Liquidity is added in two tokens in a 50/50 ratio based on value.

The `price` field in the response above represents the price of ckBTC in ckUSDT, which determines the ratio of tokens to be deposited into the pool.

**Example:** Suppose we have 1 ckBTC.

- Price: 1 ckBTC = 86,312.36 ckUSDT
- Half of the liquidity goes into the pool in ckBTC: 0.5 ckBTC
- The other half in ckUSDT: `0.5 ckBTC * 86,312.36 = 43,156.18 ckUSDT`

```rust
    let total_ckbtc: f64 = 1.0;
    let price: f64 = 86312.36;

    let ckbtc_to_swap = total_ckbtc / 2.0; // 0.5 ckBTC
    let ckbtc_to_keep = total_ckbtc - ckbtc_to_swap; // 0.5 ckBTC
    let ckusdt_received = ckbtc_to_swap * price; // 43,156.18 ckUSDT
```

Thus, we need to deposit **0.5 ckBTC and 43,156.18 ckUSDT** into the pool. To achieve this, swap half of ckBTC into ckUSDT.

### Alternative Method 1: `swap_amounts`

Another way to compute this is by calling `swap_amounts(token0, amount, token1)`, which simulates the swap.

Example:

```json
[
    "IC.mxzaz-hqaaa-aaaar-qaada-cai",
    50000000n,
    "IC.cngnf-vqaaa-aaaar-qag4q-cai"
]
```

The response field `receive_amount` shows the amount received from the swap.

### Alternative Method 2: `add_liquidity_amounts`

Another option is to call `add_liquidity_amounts(token0, amount, token1)`, where `amount` is the amount of token0 being deposited (0.5 ckBTC in this case).

Example:

```json
[
    "IC.ckBTC",
    50000000n,
    "IC.ckUSDT"
]
```

The response field `amount_1` will contain the necessary amount of the second token for the liquidity deposit.

---

## 3. Swap 50% of Liquidity

Method - `swap_amounts`

```
swap_amounts(receive_token, pay_amount, pay_token)
```

Example:

```json
[
    "IC.ryjl3-tyaaa-aaaaa-aaaba-cai",
    1000000n,
    "IC.mxzaz-hqaaa-aaaar-qaada-cai"
]
```

---

## 4. Add Liquidity to the Pool (`add_liquidity`)

After swapping, deposit the tokens into the pool.

- Approve tokens using `icrc2_approve`.
- Add liquidity via `add_liquidity(token_0, token_1, amount_0, amount_1)`.

Example:

- `amount_0 = 0.5 ckBTC`
- `amount_1 = computed ckUSDT amount from Step 2`

The response contains a `request_id` of type `AddLiquidity`, e.g., `765054n`.

We can check our position using `requests(request_id)`. The field `add_lp_token_amount` represents our LP tokens, which denote our share of the pool. These LP tokens should be stored as they are needed for withdrawal.

Method `user_balances(wallet_id)` returns a list of LP positions along with details.

---

## 5. Withdraw Liquidity from the Pool

Method `remove_liquidity_amounts(token0, token1, lpAmount)` calculates the expected token amounts returned in exchange for LP tokens.

Field `remove_lp_token_amount` specifies how many LP tokens we can redeem.

To withdraw liquidity, call:

```
remove_liquidity_async(token0, token1, lpAmount)
```

- `lpAmount` is the number of LP tokens we wish to redeem.
- To withdraw everything, provide all LP tokens.

The response contains a `request_id` of type `RemoveLiquidity`, which can be queried using `requests(request_id)`. Fields `lp_fee_0` and `lp_fee_1` represent earned fees.

All fees appear to be included in the withdrawn liquidity. There is no separate mechanism to manage them.

Finally, swap the withdrawn tokens back into the original currency (ckBTC). Now, the entire liquidity is back in its original form.


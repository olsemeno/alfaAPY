# Smart Rebalance

## Goal

The Smart Rebalance algorithm dynamically reallocates liquidity across multiple DEX pools to maximize long-term yield, while carefully managing risk, execution cost, and market conditions.

APY and other performance metrics are computed using historical snapshots of test liquidity placed in each pool. These tracked positions allow the algorithm to analyze real-world yield behavior over time — not estimates or protocol-reported numbers — and base its decisions on actual performance data.

## Core Idea

Instead of naively moving liquidity to the pool with the highest current APY, Smart Rebalance evaluates each pool using a composite scoring function that reflects:

- Smoothed APY (USD & Tokens) — averages short-term yield data to filter out noise and spikes
- Proven long-term performance — favors pools that have delivered solid results over weeks or months
- Rebalancing cost — takes into account swap fees, gas, and other overhead from moving liquidity
- Efficiency of capital — checks how much trading activity (volume) the pool generates relative to its size (TVL)
- APY volatility — penalizes pools where yield fluctuates wildly from hour to hour
- Token price stability — avoids pools with tokens that show signs of instability or potential depegging

This approach helps reduce churn, avoid noise-driven decisions, and allocate capital toward consistently high-performing, efficient, and safe liquidity pools.

## APY Calculation

### APY Inputs

We calculate APY from two perspectives to capture both value appreciation and yield in kind:

#### 1. USD-based APY

This reflects changes in the total market value of the LP position over a time period Δt (in days):

Let:

`amount_usd_initial` — initial total value in USD

`amount_usd_final` — final total value in USD after Δt

`growth_factor = amount_usd_final / amount_usd_initial`

Then:

```
APY_usd = (growth_factor ^ (365 / Δt) - 1) × 100
```

The USD value is computed as:

```
amount_usd = amount0_usd + amount1_usd
```

#### 2. Token-based APY

This captures the growth in token balances, independent of price changes:

For each token:

```
growth_factor_tokenX = amount_tokenX_final / amount_tokenX_initial
APY_tokenX = (growth_factor_tokenX ^ (365 / Δt) - 1) × 100
```

Then:

```
APY_tokens = average(APY_token0, APY_token1)
```

### SMA APY

Simple Moving Average APY over last `N` intervals (e.g. 1h snapshots):

```
SMA_APY = (1 / N) * Σ APY_i
```

- Applies short-term smoothing over the last N hours (e.g. 72h)
- Reduces the influence of temporary spikes or hourly noise

### Long-Term APY

Used as a filter, not part of the score:

```rust
if long_term_apy_usd < 0.0 {
    skip_pool(); // the pool has long-term losses
}
```

- Measures yield over a much longer period (e.g. 30 days)
- Indicates whether a pool has historically performed well

## Pool Score Function

Each pool is evaluated using a composite score:

```
score = 
    W1 × SMA_APY_usd +
    W2 × SMA_APY_tokens +
    W3 × log(TVL) +
    W4 × capital_efficiency -
    W5 × APY_volatility -
    W6 × rebalance_cost -
    W7 × token_price_volatility
```

### W1 × SMA_APY_usd

Represents smoothed short-term annualized yield (APY) of the position in USD.  
Calculated using a moving average over the last N hours (e.g. 72h), to reduce noise.

### W2 × SMA_APY_tokens

Smoothed APY based on raw token balance increase.  
Important for strategies that prioritize token farming and accumulation.

### W3 × log(TVL)

Logarithmic score of pool size (Total Value Locked).  
Encourages large, stable pools without over-penalizing small but efficient ones.

```
log_score = log10(TVL)
```

### W4 × capital_efficiency (Volume / TVL)

Measures capital capital efficiency — how much trading activity per dollar of liquidity.

```
capital_efficiency = volume_period / TVL
```

Period is strategy-dependent (e.g. 1d, 7d, 30d). High value = more fee revenue per dollar.

### W5 × APY_volatility

Penalty for unstable pools.
This metric captures how unstable or erratic the pool's returns are.

APY_volatility is calculated as the standard deviation of the pool’s hourly APY values over the past N hours (typically 24–72). It measures how much the APY fluctuates relative to its mean:

```
APY_volatility = stddev([APY_usd₁, APY_usd₂, ..., APY_usdₙ])
```

Pools with stable yields will have low APY volatility (close to 0)
Pools with sharp fluctuations will be penalized more heavily in the score
This discourages rebalancing into unpredictable pools, even if their short-term APY appears high.

### W6 × rebalance_cost

Total estimated cost of moving liquidity into the pool.

```
rebalance_cost = (fee_percent × position_value) + gas_cost
```

Where:

- `fee_percent` = remove + add DEX fees (e.g. 0.3% + 0.3%)
- `gas_cost` = fixed on-chain execution cost


Discourages frequent reallocation unless gain significantly exceeds cost.

### W7 × token_price_volatility

This penalty captures token-level risk (not LP performance), by measuring how volatile the token prices are.

Calculated as stddev of average token price in USD across snapshots.

Let `P₀ᵢ` and `P₁ᵢ` be the USD prices of token0 and token1 at snapshot `i`.
We compute the average token price per snapshot:

```
avg_priceᵢ = (P₀ᵢ + P₁ᵢ) / 2
```

Then compute the standard deviation across all snapshots:

```
token_price_volatility = stddev([avg_price₁, avg_price₂, ..., avg_priceₙ])
```

## Expected Gain

Estimated gain over expected holding period `T` (usually equal to cooldown):

```
expected_gain = (APY_target - APY_current) × balance × (T / 365)
```

## Rebalancing Logic

Smart Rebalance triggers a move only if all of the following are true:

1. Cooldown has expired
2. Pool score delta is significant:
  ```
  score_target - score_current ≥ score_threshold
  ```
3. Expected gain outweighs cost:
  ```
  expected_gain ≥ rebalance_cost × gain_cost_multiplier
  ```

### Visual Logic Flow

```
                    +------------------------+
                    |   cooldown expired?    |-- No --> [Skip]
                    +------------------------+
                                |
                                v
         +----------------------------------------------+
         | score_new - score_current > score_threshold? |-- No --> [Skip]
         +----------------------------------------------+
                                |
                                v
 +------------------------------------------------------------+
 | expected_gain > rebalance_cost × Multiplier (e.g. 2x)?     |-- No --> [Skip]
 +------------------------------------------------------------+
                                |
                                v
                        [Execute Rebalance]
```

### Rebalance Decision Struct

```rust
struct RebalanceDecision {
    should_move: bool,
    target_pool_id: Option<String>,
    score_diff: f64,
    expected_gain: f64,
    rebalance_cost: f64,
}
```

## Strategy Types

Smart Rebalance supports different strategy profiles with custom weights and thresholds:

| StrategyType       | Description                                          |
| :----------------- | :--------------------------------------------------- |
| `Conservative`     | Long-term stability, minimal APY_volatility and cost |
| `Balanced`         | Balanced between stability and yield                 |
| `Aggressive`       | Maximize returns, high-risk tolerance                |
| `TokenAccumulator` | Accumulate tokens (e.g. farming rewards)             |
| `IncentiveFarmer`  | Target boosted or incentivized pools                 |
| `StableOnly`       | Stablecoin-only exposure, minimum APY_volatility     |


## Strategy Parameters

| Strategy         | Cooldown | Threshold  | Gain/Cost | W1  | W2  | W3   | W4  | W5  | W6  | W7  |
| ---------------- | -------- | ---------- | --------- | --- | --- | ---- | --- | --- | --- | --- |
| Conservative     | 72h      | 8          | 3.0x      | 1   | 0.2 | 0.01 | 0.3 | 2   | 1.5 | 2.0 |
| Balanced         | 24–48h   | 5          | 2.0x      | 1   | 0.4 | 0.02 | 0.5 | 1   | 1.0 | 0.5 |
| Aggressive       | 6–12h    | 2          | 1.2x      | 1   | 0.6 | 0    | 1.0 | 0.2 | 0.3 | 0.1 |
| TokenAccumulator | 24–48h   | 3          | 1.5x      | 0.3 | 1.0 | 0.01 | 0.4 | 0.5 | 0.8 | 0.3 |
| IncentiveFarmer  | 12–24h   | 4          | 1.8x      | 0.8 | 0.7 | 0.01 | 0.7 | 0.6 | 0.7 | 0.4 |
| StableOnly       | 48–72h   | 6          | 2.5x      | 1   | 0.3 | 0.05 | 0.2 | 2.5 | 1.2 | 2.0 |


## Strategy Scoring Weights

### W1 – USD APY Weight

| Strategy         | W1  | Rationale                                 |
| ---------------- | --- | ----------------------------------------- |
| Conservative     | 1.0 | Stable, sustainable yield is top priority |
| Balanced         | 1.0 | Balanced focus on stable APY              |
| Aggressive       | 1.0 | High yield is main goal                   |
| TokenAccumulator | 0.3 | Less focus on USD value                   |
| IncentiveFarmer  | 0.8 | Boosted pools with attractive yield       |
| StableOnly       | 1.0 | Requires consistent USD-based return      |

### W2 – Token APY Weight

| Strategy         | W2  | Rationale                                |
| ---------------- | --- | ---------------------------------------- |
| Conservative     | 0.2 | Minimal token reward focus               |
| Balanced         | 0.4 | Partial consideration of token growth    |
| Aggressive       | 0.6 | Token rewards are a key component        |
| TokenAccumulator | 1.0 | Pure token farming strategy              |
| IncentiveFarmer  | 0.7 | Yield from farming boosted token rewards |
| StableOnly       | 0.3 | Limited but included if stable           |

### W3 – log(TVL) Weight

| Strategy         | W3   | Rationale                                   |
| ---------------- | ---- | ------------------------------------------- |
| Conservative     | 0.01 | Large pools are more trustworthy            |
| Balanced         | 0.02 | Slight preference for deeper liquidity      |
| Aggressive       | 0.00 | Doesn't care about pool size                |
| TokenAccumulator | 0.01 | Minor consideration for pool health         |
| IncentiveFarmer  | 0.01 | Slight bias to larger pools with incentives |
| StableOnly       | 0.05 | Strong preference for deeply liquid stables |

### W4 – Capital Efficiency (Volume / TVL)

| Strategy         | W4  | Rationale                                        |
| ---------------- | --- | ------------------------------------------------ |
| Conservative     | 0.3 | Fee generation is useful, but secondary          |
| Balanced         | 0.5 | Balanced between capital use and risk            |
| Aggressive       | 1.0 | Seeks high turnover pools                        |
| TokenAccumulator | 0.4 | Moderate influence from efficiency               |
| IncentiveFarmer  | 0.7 | Active pools likely to give more yield           |
| StableOnly       | 0.2 | Efficiency useful, but not at the cost of safety |

### W5 – APY Volatility

| Strategy         | W5  | Rationale                               |
| ---------------- | --- | --------------------------------------- |
| Conservative     | 2.0 | Must avoid unstable yield               |
| Balanced         | 1.0 | Allows some fluctuation                 |
| Aggressive       | 0.2 | Willing to enter volatile pools         |
| TokenAccumulator | 0.5 | Moderate risk tolerance                 |
| IncentiveFarmer  | 0.6 | Accepts fluctuations for rewards        |
| StableOnly       | 2.5 | Rejects any significant APY instability |


### W6 – Rebalance Cost

| Strategy         | W6  | Rationale                                 |
| ---------------- | --- | ----------------------------------------- |
| Conservative     | 1.5 | Moves only when benefits clearly outweigh |
| Balanced         | 1.0 | Moderate sensitivity to cost              |
| Aggressive       | 0.3 | Tolerates frequent rebalancing            |
| TokenAccumulator | 0.8 | Less frequent, but tolerable if farming   |
| IncentiveFarmer  | 0.7 | Will pay cost for extra farming rewards   |
| StableOnly       | 1.2 | Avoids unnecessary cost in low-risk pools |

### W7 – Token Price Volatility

| Strategy         | W7  | Rationale                                      |
| ---------------- | --- | ---------------------------------------------- |
| Conservative     | 2.0 | No tolerance for depegging or unstable tokens  |
| Balanced         | 0.5 | Allows moderate volatility                     |
| Aggressive       | 0.1 | Ignores price instability                      |
| TokenAccumulator | 0.3 | Allows volatility unless catastrophic          |
| IncentiveFarmer  | 0.4 | Will farm even risky tokens                    |
| StableOnly       | 2.0 | Stablecoin-only, price instability not allowed |

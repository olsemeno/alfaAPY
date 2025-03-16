<div style="display:flex;flex-direction:column;">
  <a>
    <img src="./alfaWolf.png" alt="AlfaAPY" role="presentation"/>
  </a>
</div>

# Alfa APY

Alfa APY is the first project on the Internet Computer Protocol (ICP)
that helps users find the best Annual Percentage Yield (APY) among various providers on ICP.
It functions offering users a way to maximize their returns by dynamically allocating funds to the most profitable
pools and strategies available on the ICP network.

## Features

- Dynamic Strategy Allocation: Automatically moves funds between different liquidity pools and providers to optimize
returns.
- Multiple Providers: Integrates with various providers on ICP, including Kongswap and ICPSwap.
- User-Friendly Interface: Easy-to-use interface for managing investments and tracking performance.
- Secure and Transparent: Built on the secure and transparent infrastructure of ICP.

---

## Prerequisites

Ensure you have the following tools installed before diving into development:

- **Rustup** `^v1.27.1`
- **DFX** `^v0.24.0`
- **jq** `^1.6`

> ⚠️ Note: These versions are specific for compatibility with the Internet Computer SDK.

---

## Getting Started

### Launch Local DFX

Begin by starting a local DFX instance:

```bash
dfx start --background --clean
```

### Deploy Vault

To deploy the vault canister with the specified ID, run:

```bash
dfx deploy vault --no-wallet --specified-id "hx54w-raaaa-aaaaa-qafla-cai"
```

## Integration Tests

### Prerequisites

Install these dependencies before testing:

- **NodeJS** `^v20.16.0`
- **Yarn** `^v1.22.22`

### Run Integration Tests

To run the integration tests, use:

```bash
npm i && npm run test
```

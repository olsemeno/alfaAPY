<div style="display:flex;flex-direction:column;">
  <a>
    <img src="./alfaWolf.png" alt="AlfaAPY" role="presentation"/>
  </a>
</div>



# Alfa APY

Get best APY on IC.

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

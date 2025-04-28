# ICPSwap

**SwapFactory:** [https://dashboard.internetcomputer.org/canister/4mmnk-kiaaa-aaaag-qbllq-cai](https://dashboard.internetcomputer.org/canister/4mmnk-kiaaa-aaaag-qbllq-cai)

**Swap Calculator:** [https://dashboard.internetcomputer.org/canister/phr2m-oyaaa-aaaag-qjuoq-cai](https://dashboard.internetcomputer.org/canister/phr2m-oyaaa-aaaag-qjuoq-cai)

---

# Flow

<aside>
💡

Work In Progress

</aside>

## Adding Liquidity

Doc: https://github.com/ICPSwap-Labs/docs/blob/main/02.SwapPool/Liquidity/02.Adding_Liquidity.md

…

## **Getting Amounts For Liquidity**

Doc: https://github.com/ICPSwap-Labs/docs/blob/main/02.SwapPool/Liquidity/05.Getting_Amounts_For_Liquidity.md

1. Запрос `getPool` в канистру **SwapFactory** – в response получаем `canisterId` пула
2. Запрос на `metadata` в канистру с id `canisterId` – в response получаем `sqrtPriceX96`
3. ….

---

# Requests

## Get pools

Method: `getPools`

Canister: SwapFactory

Link: [https://dashboard.internetcomputer.org/canister/xmiu5-jqaaa-aaaag-qbz7q-cai#metadata](https://dashboard.internetcomputer.org/canister/xmiu5-jqaaa-aaaag-qbz7q-cai#metadata)

## Get pool

Method: `getPool`

Canister: SwapFactory

Link: [https://dashboard.internetcomputer.org/canister/xmiu5-jqaaa-aaaag-qbz7q-cai#metadata](https://dashboard.internetcomputer.org/canister/xmiu5-jqaaa-aaaag-qbz7q-cai#metadata)

Doc: https://github.com/ICPSwap-Labs/docs/blob/main/01.SwapFactory/01.Searching_a_Pool.md

Params:

```jsx
{
	fee: nat,
	token0: {
		address: text,
		standard: text
	},
	token1: {
		address: text,
		standard: text
	}
}
```

Response:

```jsx
{
  fee = 3_000 : nat;
  key = "mxzaz-hqaaa-aaaar-qaada-cai_ryjl3-tyaaa-aaaaa-aaaba-cai_3000";
  tickSpacing = 60 : int;
  token0 = record { address = "mxzaz-hqaaa-aaaar-qaada-cai"; standard = "ICRC2" };
  token1 = record { address = "ryjl3-tyaaa-aaaaa-aaaba-cai"; standard = "ICP" };
  canisterId = principal "xmiu5-jqaaa-aaaag-qbz7q-cai";
}
```

Description:

У каждого пула отдельная канистра. Этим запросом мы можем получить `canisterId`  для пула.

## Pool metadata

Request: `metadata`

Canister: Pool canister

Link: [https://dashboard.internetcomputer.org/canister/xmiu5-jqaaa-aaaag-qbz7q-cai#metadata](https://dashboard.internetcomputer.org/canister/xmiu5-jqaaa-aaaag-qbz7q-cai#metadata)

Response:

```jsx
{
  fee = 3_000 : nat;
  key = "mxzaz-hqaaa-aaaar-qaada-cai_ryjl3-tyaaa-aaaaa-aaaba-cai_3000";
  sqrtPriceX96 = 10_495_322_873_586_518_604_502_279_289_484 : nat;
  tick = 97_731 : int;
  liquidity = 152_386_550_661 : nat;
  token0 = record { address = "mxzaz-hqaaa-aaaar-qaada-cai"; standard = "ICRC2" };
  token1 = record { address = "ryjl3-tyaaa-aaaaa-aaaba-cai"; standard = "ICP" };
  maxLiquidityPerTick = 11_505_743_598_341_114_571_880_798_222_544_994 : nat;
  nextPositionId = 1_687 : nat;
}
```
import {Ed25519KeyIdentity} from "@dfinity/identity";
import {getTypedActor} from "../util/util";
import {_SERVICE as ledgerService, ApproveArgs} from "../idl/ledger";
import {idlFactory as ledger_idl} from "../idl/ledger_idl";
import {_SERVICE as VaultType, StrategyDepositResponse, StrategyWithdrawResponse} from "../idl/vault";
import {idlFactory} from "../idl/vault_idl";
import {Principal} from "@dfinity/principal";
import {ActorSubclass} from "@dfinity/agent";
import {AccountIdentifier} from '@dfinity/ledger-icp';
import {expect} from 'chai';

export const isLocalENV = true;

describe("VR Test PROD", () => {
    const canisterId = "ownab-uaaaa-aaaap-qp2na-cai";
    const identity = "87654321876543218765432187654399";

    const icpCanisterId = "ryjl3-tyaaa-aaaaa-aaaba-cai";
    const ckBtcCanisterId = "mxzaz-hqaaa-aaaar-qaada-cai";
    const pandaCanisterId = "druyg-tyaaa-aaaaq-aactq-cai";
    const nfidwCanisterId = "mih44-vaaaa-aaaaq-aaekq-cai";
    const icsCanisterId = "ca6gz-lqaaa-aaaaq-aacwa-cai";

    // const ledgerCanisterId = icpCanisterId; // ICP
    const ledgerCanisterId = icsCanisterId; // PANDA

    let principalId: Principal;
    let memberIdentity: Ed25519KeyIdentity;
    let ledgerActor: ActorSubclass<ledgerService>
    let actorVault: ActorSubclass<VaultType>
    let balance;
    let tokenMetadata;

    beforeEach(async () => {
        memberIdentity = getIdentity(identity);
        principalId = memberIdentity.getPrincipal(); // 2ammq-nltzb-zsfkk-35abp-eprrz-eawlg-f36u7-arsde-gdhv5-flu25-iqe

        let userAddress = await principalToAddress(principalId); // 0d445feb87a73ff4dd16e744c70aede3ab806a4d6cf9a224d439d9d82489302a

        console.log("Member principal:", principalId.toText());
        console.log("Member address:", userAddress);

        ledgerActor = await getTypedActor<ledgerService>(ledgerCanisterId, memberIdentity, ledger_idl);

        // ICP balance
        // let icpLedgerActor = await getTypedActor<ledgerService>(icpCanisterId, memberIdentity, ledger_idl);
        // let icpBalance = await icpLedgerActor.icrc1_balance_of({subaccount: [], owner: principalId});
        // console.log("ICP balance:", icpBalance);

        // ckBTC balance
        // let ckBtcLedgerActor = await getTypedActor<ledgerService>(ckBtcCanisterId, memberIdentity, ledger_idl);
        // let ckBtcBalance = await ckBtcLedgerActor.icrc1_balance_of({subaccount: [], owner: principalId});
        // console.log("ckBTC balance:", ckBtcBalance);

        // PANDA balance
        // let pandaLedgerActor = await getTypedActor<ledgerService>(pandaCanisterId, memberIdentity, ledger_idl);
        // let pandaBalance = await pandaLedgerActor.icrc1_balance_of({subaccount: [], owner: principalId});
        // console.log("PANDA balance:", pandaBalance);

        actorVault = await getTypedActor<VaultType>(canisterId, memberIdentity, idlFactory);
    });

    describe(".deposit", () => {
        const strategyId = 5;
        const approveAmount = BigInt(10000000000);
        const depositAmount = BigInt(40_000_000);
        // const depositAmount = BigInt(10_000);

        it("Deposits to strategy without any liquidity", async () => {
            console.log("== START \"Deposits to strategy without any liquidity\" TEST ==");

            // Approve tokens
            await checkAndApproveTokens(approveAmount, canisterId, memberIdentity, ledgerActor);

            try {
                console.log("Deposit starting...");

                const result = await actorVault.deposit({
                    amount: depositAmount,
                    strategy_id: strategyId,
                    ledger: Principal.fromText(ledgerCanisterId)
                });

                console.log("Deposit success:", depositResp.amount, depositResp.shares, depositResp.tx_id, depositResp.request_id)

                expect(depositResp.amount).to.equal(depositAmount);
                expect(depositResp.shares).to.equal(depositAmount);
            } catch (e) {
                console.log("Deposit error:", e);
                throw new Error("Deposit failed with error: " + e);
            }
        });

        // it("Deposits to strategy with liquidity", async () => {
        // });
    });

    describe(".withdraw", () => {
        const strategyId = 5;
        const approveAmount = BigInt(10000000000);
        const depositAmount = BigInt(40_000_000);
        // const depositAmount = BigInt(50_000);

        let shares: bigint;
        let sharesToWithdraw: bigint;
        let remainingShares: bigint;

        // beforeEach(async () => {
        //     // Approve tokens
        //     await checkAndApproveTokens(approveAmount, canisterId, memberIdentity, ledgerActor);

        //     try {
        //         console.log("Deposit starting...");

        //         // Deposit tokens
        //         let depositResp: DepositResponse = await actorVault.deposit({
        //             amount: depositAmount,
        //             strategy_id: strategyId,
        //             ledger: Principal.fromText(ledgerCanisterId)
        //         });

        //         console.log("Deposit success:", depositResp.amount, depositResp.shares, depositResp.tx_id, depositResp.request_id);

        //         shares = BigInt(depositResp.shares);
        //     } catch (e) {
        //         console.log("Deposit error:", e);
        //     }
        // });

        it("Withdraws full balance", async () => {
            console.log("== START \"Withdraws full balance\" TEST ==");
            console.log("Shares:", shares);

            shares = depositAmount; // For testing without deposit
            sharesToWithdraw = shares; // All shares
            remainingShares = 0n; // No shares left

            try {
                let withdrawResp: StrategyWithdrawResponse = await actorVault.withdraw({
                    amount: sharesToWithdraw,
                    strategy_id: strategyId,
                    ledger: Principal.fromText(ledgerCanisterId)
                });
                // @ts-ignore
                console.log("Withdraw success :", withdrawResp.amount, withdrawResp.current_shares);

                expect(withdrawResp.current_shares).to.equal(0n);
            } catch (e) {
                console.log("Withdraw error: ", e);
                throw new Error("Withdraw failed with error: " + e);
            }
        });

        it("Withdraws part of balance", async () => {
            console.log("== START \"Withdraws part of balance\" TEST ==");

            shares = depositAmount; // For testing without deposit
            let sharesToWithdraw = BigInt(shares) / 2n; // 50% of shares
            // let sharesToWithdraw = BigInt(100_000_000);
            let remainingShares = BigInt(shares) - sharesToWithdraw;

            try {
                console.log("Withdraw starting...");

                let withdrawResp: StrategyWithdrawResponse = await actorVault.withdraw({
                    amount: sharesToWithdraw,
                    strategy_id: strategyId,
                    ledger: Principal.fromText(ledgerCanisterId)
                });
                // @ts-ignore
                console.log("Withdraw success :", withdrawResp.amount, withdrawResp.current_shares);

                expect(withdrawResp.current_shares).to.equal(remainingShares);
            } catch (e) {
                console.log("Withdraw error: ", e);
                throw new Error("Withdraw failed with error: " + e);
            }
        });
    });

    describe(".user_strategies", () => {
        it("Returns user strategies", async () => {
            try {
                const userStrategies = await actorVault.user_strategies(memberIdentity.getPrincipal());
                console.log("User strategies count:", userStrategies.length);

                if (userStrategies.length > 0) {
                    userStrategies.forEach(strategy => {
                        console.log(
                            `Strategy ID: ${strategy.strategy_id}\n` +
                            `Name: ${strategy.strategy_name}\n` +
                            `Initial deposit: ${strategy.initial_deposit.toString()}\n` +
                            `User shares: ${strategy.user_shares.toString()}\n` +
                            `Total shares: ${strategy.total_shares.toString()}\n`
                        );
                    });
                } else {
                    console.log("No strategies found for this user");
                }
            } catch (e) {
                console.log("User strategies error: ", e);
                throw new Error("User strategies failed with error: " + e);
            }
        });
    });

    describe(".rebalance", () => {
        // it("Rebalance", async function () {
        //     console.log("== START REBALANCE TEST ==");
        //
        //     try {
        //         let rebalance = await actorVault.rebalance();
        //         console.log("Rebalance success" + rebalance)
        //     } catch (e) {
        //         console.log(e)
        //     }
        // });
    });

    describe(".get_strategies", () => {
        it("Returns strategies", async () => {
            const strategies = await actorVault.get_strategies();
            const pandaIcpStrategy = strategies.find(strategy => strategy.id === 4);
            const pandaIcpPools = pandaIcpStrategy.pools;

            strategies.forEach(strategy => {
                console.log(
                    `Strategy ID: ${strategy.id}\n` +
                    `Name: ${strategy.name}\n` +
                    `Current pool: ${JSON.stringify(strategy.current_pool)}\n` +
                    `Total balance: ${strategy.total_balance}\n` +
                    `Total shares: ${strategy.total_shares}\n` +
                    `User shares: ${JSON.stringify(strategy.user_shares.toString())}\n`
                );
            });
        });
    });

    describe(".reset_strategy", () => {
        it("Resets strategy", async () => {
            const strategyId = 4;
            const resetResult = await actorVault.reset_strategy(strategyId);
            console.log("Reset result:", resetResult);
        });
    });

    context("ICPSWAP", () => {
        const token0 = Principal.fromText(pandaCanisterId);
        const token1 = Principal.fromText(icpCanisterId);
        const token0Fee = 10_000n;
        const token1Fee = 10_000n;

        // describe(".swap_icrc2_icpswap", () => {
        //     it("Swaps ICP for ckBTC", async () => {
        //         const amount = 620_419n;
        //         const swapResult = await actorVault.swap_icrc2_icpswap(token0, amount, token1);
        //         console.log("Swap result:", swapResult);
        //     });
        // });

        // describe(".get_icpswap_quote", () => {
        //     it("Returns ICP/ckBTC quote", async () => {
        //         const amount = 300_000_000n;

        //         const quote = await actorVault.get_icpswap_quote(token0, token1, amount);
        //         console.log("ICP/ckBTC quote:", quote);
        //     });
        // });

        // describe(".swap_icpswap", () => {
        //     it("Swaps ICP for USDC", async () => {
        //         const amount = 270_000n;

        //         const quote = await actorVault.get_icpswap_quote(token0, token1, amount);
        //         console.log("ICP/ckBTC quote:", quote);

        //         const swapResult = await actorVault.swap_icpswap(token0, token1, amount);
        //         console.log("Swap result:", swapResult);
        //     });
        // });

        // Withdraw token from ICPSwap canister
        describe(".icpswap_withdraw_1", () => {
            it("Withdraws", async () => {
                const amount = 5_577_528_681n;

                let withdrawResult = await actorVault.icpswap_withdraw(token0, 5_577_528_681n, token1Fee);
                console.log("Withdraw result:", withdrawResult);

                withdrawResult = await actorVault.icpswap_withdraw(token1, 353_486n, token1Fee);
                console.log("Withdraw result:", withdrawResult);
            });
        });

        // describe(".icpswap_add_liquidity", () => {
        //     it("Adds liquidity to ICP/ckBTC pool", async () => {
        //         const amount = 500_000n; // 0.005 ICP

        //         const addLiquidityResult = await actorVault.icpswap_add_liquidity(amount, token0, token1);
        //         console.log("Add liquidity result:", addLiquidityResult);
        //     });
        // });

        // describe(".icpswap_withdraw_from_pool", () => {
        //     it("Withdraws ICP from ICP/ckBTC pool", async () => {
        //         const shares = 1000000000000000000n;
        //         const total_shares = 1000000000000000000n;

        //         const withdrawResult = await actorVault.icpswap_withdraw_from_pool(total_shares, shares, token0, token1);
        //         console.log("Withdraw result:", withdrawResult);
        //     });
        // });
    });

    // context("KONGSWAP", () => {
    //     const token0 = Principal.fromText(pandaCanisterId);
    //     const token1 = Principal.fromText(icpCanisterId);

    //     describe(".get_kongswap_quote", () => {
    //         it("Returns ICP/ckBTC quote", async () => {
    //             const amount = 50_000_000n;

    //             const quote = await actorVault.get_kongswap_quote(token0, token1, amount);
    //             console.log("quote:", quote);
    //         });
    //     });

    //     describe(".swap_kongswap", () => {
    //         it("Swaps ICP for USDC", async () => {
    //             const amount = 900_000n;

    //             const quote = await actorVault.get_kongswap_quote(token0, token1, amount);
    //             console.log("ICP/ckBTC quote:", quote);

    //             const swapResult = await actorVault.swap_kongswap(token0, token1, amount);
    //             console.log("Swap result:", swapResult);
    //         });
    //     });

    //     describe(".kong_add_liquidity", () => {
    //         it("Adds liquidity to ICP/ckBTC pool", async () => {
    //             const amount = 100_000_000n;

    //             const addLiquidityResult = await actorVault.kong_add_liquidity(amount, token0, token1);
    //             console.log("Add liquidity result:", addLiquidityResult);
    //         });
    //     });

    //     describe(".kong_withdraw_from_pool", () => {
    //         it("Withdraws ICP from ICP/ckBTC pool", async () => {
    //             const shares = 1000000000000000000n;
    //             const total_shares = 1000000000000000000n;

    //             const withdrawResult = await actorVault.kong_withdraw_from_pool(total_shares, shares, token0, token1);
    //             console.log("Withdraw result:", withdrawResult);
    //         });
    //     });
    // });
});

export const getIdentity = (seed: string): Ed25519KeyIdentity => {
    let seedEncoded = new TextEncoder().encode(seed);

    return Ed25519KeyIdentity.generate(seedEncoded);
};

export const checkAndApproveTokens = async (
    amount: bigint,
    canisterId: string,
    memberIdentity: Ed25519KeyIdentity,
    ledgerActor: ActorSubclass<ledgerService>
) => {
    let approveArgs: ApproveArgs = {
        amount: amount,
        spender: {
            owner: Principal.fromText(canisterId),
            subaccount: []
        },
        fee: [],
        memo: [],
        from_subaccount: [],
        created_at_time: [],
        expected_allowance: [],
        expires_at: []
    };

    console.log("Approve tokens starting...");

    // Approve tokens
    const approveResponse = await ledgerActor.icrc2_approve(approveArgs);
    console.log("IRC2 approve:", approveResponse);

    // Check allowance
    const allowanceResponse = await ledgerActor.icrc2_allowance({
        account: {
            owner: memberIdentity.getPrincipal(),
            subaccount: []
        },
        spender: {
            owner: Principal.fromText(canisterId),
            subaccount: []
        }
    });

    console.log("Allowance:", allowanceResponse);

    if (allowanceResponse.allowance < amount) {
        throw new Error("Insufficient allowance");
    }
}

export const principalToAddress = async (principalId: Principal): Promise<string> => {
    const accountIdentifier = AccountIdentifier.fromPrincipal({
        principal: principalId,
        subAccount: undefined
    });

    return accountIdentifier.toHex();
}

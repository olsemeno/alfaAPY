import { Ed25519KeyIdentity } from "@dfinity/identity";
import { getTypedActor } from "../util/util";
import { _SERVICE as ledgerService, ApproveArgs } from "../idl/ledger";
import { idlFactory as ledger_idl } from "../idl/ledger_idl";
import { _SERVICE as VaultType, DepositResponse, WithdrawResponse } from "../idl/vault";
import { idlFactory } from "../idl/vault_idl";
import { Principal } from "@dfinity/principal";
import { ActorSubclass } from "@dfinity/agent";
import { AccountIdentifier } from '@dfinity/ledger-icp';
import { expect } from 'chai';

export const isLocalENV = true;

describe("VR Test PROD", () => {
    const canisterId = "hx54w-raaaa-aaaaa-qafla-cai";
    const identity = "87654321876543218765432187654399";
    const ledgerCanisterId = "ryjl3-tyaaa-aaaaa-aaaba-cai";
    let principalId: Principal;
    let memberIdentity: Ed25519KeyIdentity;
    let ledgerActor: ActorSubclass<ledgerService>
    let actorVault: ActorSubclass<VaultType>
    let balance;

    beforeEach(async () => {
        memberIdentity  = getIdentity(identity);
        principalId = memberIdentity.getPrincipal(); //2ammq-nltzb-zsfkk-35abp-eprrz-eawlg-f36u7-arsde-gdhv5-flu25-iqe

        let userAddress = await principalToAddress(principalId); // 0d445feb87a73ff4dd16e744c70aede3ab806a4d6cf9a224d439d9d82489302a

        console.log("Member principal:", principalId.toText());
        console.log("Member address:", userAddress);

        ledgerActor = await getTypedActor<ledgerService>(ledgerCanisterId, memberIdentity, ledger_idl);
        balance = await ledgerActor.icrc1_balance_of({subaccount: [], owner: principalId});

        console.log("Balance:", balance);

        actorVault = await getTypedActor<VaultType>(canisterId, memberIdentity, idlFactory);
    });

    describe(".accept_investment", () => {
        const strategyId = 2;
        const approveAmount = BigInt(30000000);
        const depositAmount = BigInt(1000000);

        it("Deposits to strategy without any liquidity", async () => {
            console.log("== START \"Deposits to strategy without any liquidity\" TEST==");

            // Approve tokens
            await checkAndApproveTokens(approveAmount, canisterId, memberIdentity, ledgerActor);

            try {
                console.log("Deposit starting...");

                let depositResp: DepositResponse = await actorVault.accept_investment({
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
        const strategyId = 2;
        const approveAmount = BigInt(30000000);
        const depositAmount = BigInt(1000000);
        let shares: bigint;
        let sharesToWithdraw: bigint;
        let remainingShares: bigint;

        beforeEach(async () => {
            // Approve tokens
            await checkAndApproveTokens(approveAmount, canisterId, memberIdentity, ledgerActor);

            try {
                console.log("Deposit starting...");

                // Deposit tokens
                let depositResp: DepositResponse = await actorVault.accept_investment({
                    amount: depositAmount,
                    strategy_id: strategyId,
                    ledger: Principal.fromText(ledgerCanisterId)
                });

                console.log("Deposit success:", depositResp.amount, depositResp.shares, depositResp.tx_id, depositResp.request_id);

                shares = depositResp.shares;
            } catch (e) {
                console.log("Deposit error:", e);
            }
        });

        it("Withdraws full balance", async () => {
            console.log("== START \"Withdraws full balance\" TEST==");

            sharesToWithdraw = shares; // All shares
            remainingShares = 0n; // No shares left

            try {
                console.log("Withdraw starting...");
                let withdrawResp: WithdrawResponse = await actorVault.withdraw({
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
            console.log("== START \"Withdraws half balance\" TEST ==");

            let sharesToWithdraw = shares / BigInt(2); // 50% of shares
            let remainingShares = shares - sharesToWithdraw;

            try {
                console.log("Withdraw starting...");
                let withdrawResp: WithdrawResponse = await actorVault.withdraw({
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

    describe(".user_balance_all", () => {
        it("Returns user balance", async () => {
            try {
                const userBalance = await actorVault.user_balance_all(memberIdentity.getPrincipal());
                console.log("User balance:", userBalance);
            } catch (e) {
                console.log("User balance error: ", e);
                throw new Error("User balance failed with error: " + e);
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
                        console.log(`Strategy ID: ${strategy.strategy_id}, Name: ${strategy.strategy_name}, User shares: ${strategy.user_shares.toString()}, Total shares: ${strategy.total_shares.toString()}`);
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

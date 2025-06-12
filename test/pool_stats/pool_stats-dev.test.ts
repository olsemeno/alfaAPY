import {Ed25519KeyIdentity} from "@dfinity/identity";
import {getTypedActor} from "../util/util";
import {_SERVICE as ledgerService, ApproveArgs} from "../idl/ledger";
import {idlFactory as ledger_idl} from "../idl/ledger_idl";
import {_SERVICE as PoolStatsType} from "../idl/pool_stats";
import {idlFactory} from "../idl/vault_idl";
import {Principal} from "@dfinity/principal";
import {ActorSubclass} from "@dfinity/agent";
import {AccountIdentifier} from '@dfinity/ledger-icp';
import {expect} from 'chai';
import canisterIds from '../../canister_ids.json';

export const isLocalENV = true;

describe("Pool Stats Test DEV", () => {
    const canisterId = canisterIds.pool_stats.dev;
    const identity = "87654321876543218765432187654399";

    const pandaCanisterId = "druyg-tyaaa-aaaaq-aactq-cai";

    const ledgerCanisterId = pandaCanisterId;

    let principalId: Principal;
    let memberIdentity: Ed25519KeyIdentity;
    let ledgerActor: ActorSubclass<ledgerService>
    let actorPoolStats: ActorSubclass<PoolStatsType>

    beforeEach(async () => {
        memberIdentity = getIdentity(identity);
        principalId = memberIdentity.getPrincipal(); // 2ammq-nltzb-zsfkk-35abp-eprrz-eawlg-f36u7-arsde-gdhv5-flu25-iqe

        let userAddress = await principalToAddress(principalId); // 0d445feb87a73ff4dd16e744c70aede3ab806a4d6cf9a224d439d9d82489302a

        console.log("Member principal:", principalId.toText());
        console.log("Member address:", userAddress);

        ledgerActor = await getTypedActor<ledgerService>(ledgerCanisterId, memberIdentity, ledger_idl);
        actorPoolStats = await getTypedActor<PoolStatsType>(canisterId, memberIdentity, idlFactory);
    });

    describe(".create_pool_snapshot", () => {
        it("Creates pool snapshot", async () => {
            console.log("== START \"Creates pool snapshot\" TEST ==");

            const poolId = "1";
            const result = await actorPoolStats.create_pool_snapshot(poolId);

            console.log("Pool snapshot:", result);
        });
    });

    describe(".deposit", () => {
        it("Deposit liquidity to pool", async () => {
            console.log("== START \"Deposit liquidity to pool\" TEST ==");

            const poolId = "1";
            const approveAmount = BigInt(10000000000);
            const depositAmount = BigInt(40_000_000);
        
            await checkAndApproveTokens(approveAmount, canisterId, memberIdentity, ledgerActor);

            try {
                console.log("Deposit starting...");

                const result = await actorPoolStats.add_liquidity_to_pool(poolId, depositAmount);

                console.log("Pool snapshot:", result);
            } catch (e) {
            }
        });
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

import {Ed25519KeyIdentity} from "@dfinity/identity";
import {getTypedActor} from "../util/util";
import {_SERVICE as ledgerService, ApproveArgs} from "../idl/ledger";
import {idlFactory as ledger_idl} from "../idl/ledger_idl";
import {_SERVICE as VaultType, DepositResponse, Result} from "../idl/vault";
import {idlFactory} from "../idl/vault_idl";
import {Principal} from "@dfinity/principal";
import {ActorSubclass} from "@dfinity/agent";

export const isLocalENV = true;

//2ammq-nltzb-zsfkk-35abp-eprrz-eawlg-f36u7-arsde-gdhv5-flu25-iqe
describe("VR Test PROD", () => {
    let canister_id = "hx54w-raaaa-aaaaa-qafla-cai";
    let identity = "87654321876543218765432187654399";
    let ledger_canister_id = "ryjl3-tyaaa-aaaaa-aaaba-cai";
    let member_identity: Ed25519KeyIdentity;
    let ledgerActor : ActorSubclass<ledgerService>
    let actorVault : ActorSubclass<VaultType>
    let balance;

    beforeEach(async function () {
        member_identity  = getIdentity(identity)
        console.log(member_identity.getPrincipal().toText());

        ledgerActor = await getTypedActor<ledgerService>(ledger_canister_id, member_identity, ledger_idl);
        balance = await ledgerActor.icrc1_balance_of({subaccount: [], owner: member_identity.getPrincipal()});

        console.log(balance)

        actorVault = await getTypedActor<VaultType>(canister_id, member_identity, idlFactory);
    });

    it("Deposit balance", async function () {
        console.log("START DEPOSIT TEST");

        let approveargs: ApproveArgs = {
            amount: BigInt(30000000),
            spender: {
                owner: Principal.fromText(canister_id),
                subaccount: []
            },
            fee: [],
            memo: [],
            from_subaccount: [],
            created_at_time: [],
            expected_allowance: [],
            expires_at: []
        }

        let icrc2approve = await ledgerActor.icrc2_approve(approveargs);

        console.log(icrc2approve);

        let allowance = await ledgerActor.icrc2_allowance({
            account: {
                owner: member_identity.getPrincipal(),
                subaccount: []
            },
            spender: {
                owner: Principal.fromText(canister_id),
                subaccount: []
            }
        });

        console.log(allowance);

        try {
            let deposit: DepositResponse = await actorVault.accept_investment({
                amount: BigInt(1000000),
                strategy_id: 2,
                ledger: Principal.fromText("ryjl3-tyaaa-aaaaa-aaaba-cai")
            });
            console.log("Deposit success:", deposit.amount, deposit.shares, deposit.tx_id, deposit.request_id)
        } catch (e) {
            console.log(e)
        }
    });

    it("Withdraw balance", async function () {
        console.log("START WITHDRAW TEST");

        try {
            let withdraw: Result = await actorVault.withdraw({
                amount: BigInt(500000),
                strategy_id: 2,
                ledger: Principal.fromText("ryjl3-tyaaa-aaaaa-aaaba-cai")
            });
            // @ts-ignore
            console.log("Withdraw success :", withdraw.Ok)
        } catch (e) {
            console.log(e)
        }
    });

    // it("Rebalance", async function () {
    //     console.log("START REBALANCE TEST");
    //
    //     try {
    //         let rebalance = await actorVault.rebalance();
    //         console.log("Rebalance success" + rebalance)
    //     } catch (e) {
    //         console.log(e)
    //     }
    // });
});


export const getIdentity = (seed: string): Ed25519KeyIdentity => {
    let seedEncoded = new TextEncoder().encode(seed);

    return Ed25519KeyIdentity.generate(seedEncoded);
};
import {Ed25519KeyIdentity} from "@dfinity/identity";
import {getTypedActor} from "../util/util";
import {_SERVICE as ledgerService, ApproveArgs} from "../idl/ledger";
import {idlFactory as ledger_idl} from "../idl/ledger_idl";
import {_SERVICE as VaultType} from "../idl/vault";
import {idlFactory} from "../idl/vault_idl";
import {Principal} from "@dfinity/principal";

export const isLocalENV = true


describe("VR Test PROD" , () => {
    let canister_id = "hx54w-raaaa-aaaaa-qafla-cai"


    it("Deposit balance", async function () {
        let member_identity = getIdentity("87654321876543218765432187654399")

        console.log(member_identity.getPrincipal().toText())
        let actor = await getTypedActor<ledgerService>("ryjl3-tyaaa-aaaaa-aaaba-cai", member_identity, ledger_idl);
        let balance = await actor.icrc1_balance_of(
            {subaccount: [], owner: member_identity.getPrincipal()})
        console.log(balance)

        let approveargs: ApproveArgs = {
            amount: BigInt(200000000),
            spender: {
                owner: Principal.fromText("hx54w-raaaa-aaaaa-qafla-cai"),
                subaccount: []
            },
            fee: [],
            memo: [],
            from_subaccount: [],
            created_at_time: [],
            expected_allowance: [],
            expires_at: []
        }
        let icrc2approve = await actor.icrc2_approve(approveargs)
        console.log(icrc2approve)
        let allowance = await actor.icrc2_allowance({
            account: {
                owner: member_identity.getPrincipal(),
                subaccount: []
            },
            spender: {
                owner: Principal.fromText("hx54w-raaaa-aaaaa-qafla-cai"),
                subaccount: []
            }

        })

        console.log(allowance)

        let actorVault = await getTypedActor<VaultType>(canister_id, member_identity, idlFactory);

        try {
            let deposit = await  actorVault.accept_investment({amount: BigInt(100), strategy_id: 2,  ledger: Principal.fromText("ryjl3-tyaaa-aaaaa-aaaba-cai")})
            console.log("Deposit success" + deposit)
        }catch (e) {
            console.log(e)
        }



    });


})


export const getIdentity = (seed: string): Ed25519KeyIdentity => {
    let seedEncoded = new TextEncoder().encode(seed);
    return Ed25519KeyIdentity.generate(seedEncoded);
};
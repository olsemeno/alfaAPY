import {expect} from "chai";
import {getTypedActor} from "../util/util";
import {_SERVICE as VaultType} from "../idl/vault"
import {DFX} from "../constants/dfx.const";
import {Ed25519KeyIdentity} from "@dfinity/identity";
import {idlFactory} from "../idl/vault_idl";
import {idlFactory as ledger_idl} from "../idl/ledger_idl";
import {_SERVICE as ledgerService, ApproveArgs} from "../idl/ledger";
import {Principal} from "@dfinity/principal";

export const isLocalENV = true


describe("VR Test", () => {
    let canister_id
    before(async () => {
        // DFX.INIT();
        // console.log(execute(`dfx deploy vault`))
        canister_id = DFX.GET_CANISTER_ID("vault");
    });

    after(() => {
        // DFX.STOP();
    });

    it("Get config", async function () {
        let actor = await getTypedActor<VaultType>(canister_id, Ed25519KeyIdentity.generate(), idlFactory);
        let config = await actor.get_config();
        expect(config.controllers).not.null;
    });


    it("User balance", async function () {
        let member_identity = getIdentity("87654321876543218765432187654322")
        console.log(member_identity.getPrincipal().toText())

        await console.log(DFX.LEDGER_FILL_BALANCE(member_identity.getPrincipal().toText()))
        let actor = await getTypedActor<ledgerService>("ryjl3-tyaaa-aaaaa-aaaba-cai", member_identity, ledger_idl);
        let balance = await actor.icrc1_balance_of(
            {subaccount: [], owner: member_identity.getPrincipal()})

        console.log(balance)
        let approveargs: ApproveArgs = {
            amount: BigInt(200000000),
            spender: {
                owner: Principal.fromText("bd3sg-teaaa-aaaaa-qaaba-cai"),
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
                owner: Principal.fromText("bd3sg-teaaa-aaaaa-qaaba-cai"),
                subaccount: []
            }

        })
        let actorVault = await getTypedActor<VaultType>(canister_id, member_identity, idlFactory);
        let accept = await actorVault.accept_investment({
            ledger: Principal.fromText("ryjl3-tyaaa-aaaaa-aaaba-cai"), amount: BigInt(100000000),
            strategy_id: 1
        })
        let balance2 = await actor.icrc1_balance_of({
            subaccount: [],
            owner: Principal.fromText("bd3sg-teaaa-aaaaa-qaaba-cai")
        })

        console.log(balance2)

        let withdraw = await actorVault.withdraw({
            ledger: Principal.fromText("ryjl3-tyaaa-aaaaa-aaaba-cai"), amount: BigInt(100000000), strategy_id: 1
        })

        console.log(withdraw)
        let balance3 = await actor.icrc1_balance_of({
            subaccount: [],
            owner: Principal.fromText("bd3sg-teaaa-aaaaa-qaaba-cai")
        })
        console.log(balance3)

        expect(balance3 < balance2).is.true


    });


})


export const getIdentity = (seed: string): Ed25519KeyIdentity => {
    let seedEncoded = new TextEncoder().encode(seed);
    return Ed25519KeyIdentity.generate(seedEncoded);
};
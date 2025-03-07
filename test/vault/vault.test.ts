import {expect} from "chai";
import {getTypedActor} from "../util/util";
import {_SERVICE as VaultType} from "../idl/vault"
import {DFX} from "../constants/dfx.const";
import {Ed25519KeyIdentity} from "@dfinity/identity";
import {idlFactory} from "../idl/vault_idl";
import {execute} from "../util/call.util";

export const isLocalENV = true


describe("VR Test", () => {
    let canister_id
    before(async () => {
        // DFX.INIT();
        console.log(execute(`dfx deploy vault`))
        canister_id = DFX.GET_CANISTER_ID("vault");
    });

    after(() => {
        // DFX.STOP();
    });

    it("Get config", async function () {
        let actor =  await getTypedActor<VaultType>(canister_id, Ed25519KeyIdentity.generate(), idlFactory);
        let config = await actor.get_config();
        expect(config.controllers).not.null;
    });



})
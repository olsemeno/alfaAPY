import * as Agent from "@dfinity/agent";
import { Agent as DfinityAgent } from "@dfinity/agent";

import { idlFactory as icrc1IDL } from "../../../idl/icrc1_idl";
import {
  _SERVICE as ICRC1Service,
  Icrc1TransferResult,
  TransferArg,
} from "../../../idl/icrc1";

/*
 PTAL "Get index data" test in icrc1/index.spec.ts
 * rootPrincipalId: the principal id of the account im.getAccount().principalId
 * publicKey: the public key returned by lambda ecdsa.ts getPublicKey() => convert to principal with Ed25519JSONableKeyIdentity
 * maxResults: the maximum number of transactions to return
 */

export async function transferICRC1(
  agent: DfinityAgent,
  iCRC1Canister: string,
  args: TransferArg
): Promise<Icrc1TransferResult> {
  const actor = Agent.Actor.createActor<ICRC1Service>(icrc1IDL, {
    canisterId: iCRC1Canister,
    agent,
  });
  return await actor.icrc1_transfer(args);
}

export * from "./service";

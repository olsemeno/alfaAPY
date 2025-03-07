import {Actor, HttpAgent, Identity} from "@dfinity/agent";
import {IDL} from "@dfinity/candid";
const localhost: string = "http://127.0.0.1:8000";
import * as Agent from "@dfinity/agent";
import {isLocalENV} from "../vault/vault.test";

export async function getTypedActor<T>(
    imCanisterId: string,
    identity: Identity,
    idl: IDL.InterfaceFactory
): Promise<Agent.ActorSubclass<T>> {
    const agent: HttpAgent = await HttpAgent.create({host: localhost, identity: identity, shouldFetchRootKey: isLocalENV});
    return Actor.createActor(idl, {agent, canisterId: imCanisterId});
}
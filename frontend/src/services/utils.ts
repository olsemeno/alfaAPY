/* eslint-disable @typescript-eslint/no-explicit-any */
import {
  Actor, Agent as DfinityAgent,
  HttpAgent
} from "@dfinity/agent";
import { IDL } from "@dfinity/candid";
import * as Agent from "@dfinity/agent";

export function hasOwnProperty<
  X extends Record<string, unknown>,
  Y extends PropertyKey
>(obj: X, prop: Y): obj is X & Record<Y, unknown> {
  return Object.prototype.hasOwnProperty.call(obj, prop);
}

export async function getTypedActor<T>(
  imCanisterId: string,
  agent: DfinityAgent,
  idl: IDL.InterfaceFactory
): Promise<Agent.ActorSubclass<T>> {
  return Actor.createActor(idl, { agent, canisterId: imCanisterId });
}

export async function getAnonActor<T>(canisterId: string, idl: IDL.InterfaceFactory): Promise<Agent.ActorSubclass<T>> {
  const annonymousAgent = await HttpAgent.create({ host: "https://ic0.app" });
  return getTypedActor<T>(canisterId, annonymousAgent, idl);
}


export async function getDfinityActor<T>(agent: DfinityAgent, canisterId: string, idl: IDL.InterfaceFactory): Promise<Agent.ActorSubclass<T>> {
  return getTypedActor<T>(canisterId, agent, idl);
}
import { Actor, ActorConfig, ActorSubclass } from "@dfinity/agent";
import { InterfaceFactory } from "@dfinity/candid/lib/cjs/idl";
import { Principal } from "@dfinity/principal";
import { agent } from "./agent";

export function createActor<T>(
  canisterId: string | Principal,
  factory: InterfaceFactory,
  config?: Partial<ActorConfig>
): ActorSubclass<T> {
  return Actor.createActor(factory, { canisterId, agent, ...config });
}

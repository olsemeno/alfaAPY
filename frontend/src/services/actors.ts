import { InterfaceFactory } from "@dfinity/candid/lib/cjs/idl";
import { Principal } from "@dfinity/principal";
import * as Agent from "@dfinity/agent";
import { AgentWithRetry } from "./agent";

export const agentBaseConfig = { host: "https://ic0.app" };

export function actorBuilder<T>(
  canisterId: string | Principal,
  factory: InterfaceFactory,
  config?: Partial<Agent.ActorConfig>
): Agent.ActorSubclass<T> {
  return Agent.Actor.createActor(factory, {
    canisterId,
    agent: new AgentWithRetry({ ...agentBaseConfig }),
    ...config,
  });
}

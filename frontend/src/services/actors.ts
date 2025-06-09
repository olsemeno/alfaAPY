import { InterfaceFactory } from "@dfinity/candid/lib/cjs/idl";
import { Principal } from "@dfinity/principal";
import * as Agent from "@dfinity/agent";
import { AgentWithRetry } from "./agent";

export const agentBaseConfig = { host: "https://ic0.app" };

export function actorBuilder<T>({
  canisterId,
  factory,
  config,
  ...params
}: {
  canisterId: string | Principal;
  factory: InterfaceFactory;
  config?: Partial<Agent.ActorConfig>;
  agent?: Agent.Agent;
}): Agent.ActorSubclass<T> {
  const agent = params.agent ?? new AgentWithRetry({ ...agentBaseConfig });
  return Agent.Actor.createActor(factory, {
    canisterId,
    agent,
    ...config,
  });
}

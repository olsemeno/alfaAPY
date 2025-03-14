import {_SERVICE as VaultType, DepositResponse, WithdrawResponse, StrategyResponse} from "../idl/vault";
import {_SERVICE as Kong, PoolsReply, PoolsResult} from "../idl/kong_backend";
import {Actor, ActorSubclass, AnonymousIdentity, HttpAgent, Identity} from "@dfinity/agent";
import {idlFactory} from "../idl/vault_idl";
import {idlFactory as KongIDL} from "../idl/kong_backend_idl";
import {IDL} from "@dfinity/candid";
import * as Agent from "@dfinity/agent";
import {Principal} from "@dfinity/principal";

export const alfaACanister = "hx54w-raaaa-aaaaa-qafla-cai";
export const kongCanister = "hx54w-raaaa-aaaaa-qafla-cai";

export class StrategyWrapper {
    private actor: ActorSubclass<VaultType>
    private kongActor: ActorSubclass<Kong>

    protected constructor(actor: ActorSubclass<VaultType>, kongActor: ActorSubclass<Kong>) {
        this.actor = actor;
        this.kongActor = kongActor;
    }

    static async build(): Promise<StrategyWrapper> {
        let actor = await getTypedActor<VaultType>(alfaACanister, new AnonymousIdentity(), idlFactory);
        let kongActor = await getTypedActor<Kong>(kongCanister, new AnonymousIdentity(), KongIDL);
        return new StrategyWrapper(actor, kongActor)
    }

    public async get_strategies(): Promise<Array<StrategyResponse>> {
        return this.actor.get_strategies()
    }

    //todo accept identity-kit actor
    public async withdraw(strategy_id: number, ledger: string, amount: bigint): Promise<WithdrawResponse> {
        return this.actor.withdraw({strategy_id, ledger: Principal.fromText(ledger), amount})
    }

    //todo accept identity-kit actor
    public async accept_investment(strategy_id: number, ledger: string, amount: bigint): Promise<DepositResponse> {
        return this.actor.accept_investment({strategy_id, ledger: Principal.fromText(ledger), amount})
    }

    public async get_pool_data(pools_symbols:Array<String>): Promise<any> {
        return this.kongActor.pools([])
            .then((result: PoolsResult) => {
                // @ts-ignore
                let  pools: PoolsReply = result.Ok
                return pools.pools.filter((pool) => pools_symbols.includes(pool.symbol))
            })
    }
}

export async function getTypedActor<T>(
    imCanisterId: string,
    identity: Identity,
    idl: IDL.InterfaceFactory
): Promise<Agent.ActorSubclass<T>> {
    let host = "https://ic0.app";
    const agent: HttpAgent = await HttpAgent.create({host, identity: identity, shouldFetchRootKey: false});
    return Actor.createActor(idl, {agent, canisterId: imCanisterId});
}

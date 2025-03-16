import * as Agent from "@dfinity/agent"
import { HttpAgent } from "@dfinity/agent"
import { Principal } from "@dfinity/principal"

import { IIcrc1Pair } from "../i-icrc-pair"

import { idlFactory as icrc1IDL } from "../../../../../idl/icrc1_idl"
import { _SERVICE as ICRC1ServiceIDL } from "../../../../../idl/icrc1"
import { idlFactory as icrc1IndexIDL } from "../../../../../idl/index-icrc1_idl"
import { _SERVICE as ICRCIndex } from "../../../../../idl/index-icrc1"
import { ICRC1Data, ICRC1Error } from "../../types"
import { agentBaseConfig } from "../../../../actors"

export class Icrc1Pair implements IIcrc1Pair {
  private readonly ledger: string
  private readonly index: string | undefined

  constructor(ledger: string, index: string | undefined) {
    this.ledger = ledger
    this.index = index
  }

  async validateStandard() {
    const actor = Agent.Actor.createActor<ICRC1ServiceIDL>(icrc1IDL, {
      canisterId: this.ledger,
      agent: new HttpAgent({ ...agentBaseConfig }),
    })
    try {
      const standards = await actor.icrc1_supported_standards()
      const isICRC1: boolean = standards
        .map((standard) => standard.name)
        .some((name) => name === "ICRC-1")
      if (!isICRC1) {
        throw new ICRC1Error(
          "This does not appear to be an ICRC-1 compatible ledger canister.",
        )
      }
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    } catch (e) {
      throw new ICRC1Error(
        "This does not appear to be an ICRC-1 compatible ledger canister.",
      )
    }
  }

  async validateIndexCanister() {
    if (this.index) {
      try {
        const expectedLedgerId = await this.getLedgerIdFromIndexCanister(
          this.index,
        )
        if (expectedLedgerId.toText() !== this.ledger) {
          throw new ICRC1Error("Ledger canister does not match index canister.")
        }
      } catch (e) {
        if (e instanceof ICRC1Error) {
          throw e
        }
        throw new ICRC1Error(
          "This does not appear to be an ICRC-1 compatible index canister.",
        )
      }
    }
  }

  async getICRC1Data(publicKey: string): Promise<ICRC1Data> {
    const [metadata, balance] = await Promise.all([
      this.getMetadata(),
      this.getBalance(publicKey),
    ])

    return {
      owner: Principal.fromText(publicKey),
      balance,
      canisterId: this.ledger,
      ...metadata,
    }
  }

  async getBalance(principal: string): Promise<bigint> {
    const actor = Agent.Actor.createActor<ICRC1ServiceIDL>(icrc1IDL, {
      canisterId: this.ledger,
      agent: new HttpAgent({ ...agentBaseConfig }),
    })
    return await actor.icrc1_balance_of({
      subaccount: [],
      owner: Principal.fromText(principal),
    })
  }

  async getMetadata() {
    const actor = Agent.Actor.createActor<ICRC1ServiceIDL>(icrc1IDL, {
      canisterId: this.ledger,
      agent: new HttpAgent({ ...agentBaseConfig }),
    })
    const metadata = await actor.icrc1_metadata()
    let name = ""
    let symbol = ""
    let logo: string | undefined = undefined
    let decimals = 0
    let fee = BigInt(0)

    //TODO one day
    for (let i = 0; i < metadata.length; i++) {
      const data = metadata[i]
      if (data[0] === "icrc1:name") {
        const val = data[1] as { Text: string }
        name = val.Text
      } else if (data[0] === "icrc1:symbol") {
        const val = data[1] as { Text: string }
        symbol = val.Text
      } else if (data[0] === "icrc1:decimals") {
        const val = data[1] as { Nat: bigint }
        decimals = Number(val.Nat)
      } else if (data[0] === "icrc1:fee") {
        const val = data[1] as { Nat: bigint }
        fee = val.Nat
      } else if (data[0] === "icrc1:logo") {
        const val = data[1] as { Text: string }
        logo = val.Text
      }
    }

    return {
      name,
      symbol,
      logo,
      decimals,
      fee,
      canister: this.ledger,
    }
  }

  private getLedgerIdFromIndexCanister(
    indexCanister: string,
  ): Promise<Principal> {
    const indexActor = Agent.Actor.createActor<ICRCIndex>(icrc1IndexIDL, {
      canisterId: indexCanister,
      agent: new HttpAgent({ ...agentBaseConfig }),
    })
    return indexActor.ledger_id()
  }
}

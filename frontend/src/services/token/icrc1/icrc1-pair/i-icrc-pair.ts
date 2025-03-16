import { ICRC1Data } from "../types"

export interface IIcrc1Pair {
  validateStandard(): Promise<void>

  validateIndexCanister(): Promise<void>

  getICRC1Data(publicKey: string): Promise<ICRC1Data>

  getBalance(principal: string): Promise<bigint>

  getMetadata(): Promise<{
    name: string
    symbol: string
    logo?: string
    decimals: number
    fee: bigint
  }>
}

import { Quote } from "./quote"
import { SwapName } from "./types/enums"

export interface Shroff {
  getSwapName(): SwapName
  setQuote(quote: Quote): void
  getTargets(): string[]
  getQuote(amount: string): Promise<Quote>
  swap(): Promise<void>
}

import { ICRC1 } from "../../idl/icrc1_oracle";
import { StrategyResponse } from "../../services/strategies/idl/vault";

export function getTokenLogo(symbol: string, tokens: ICRC1[]) {
  return tokens.find((token) => token.symbol === symbol)?.logo?.[0] ?? "";
}

export function getStrategyTokenLogos(
  strategy: StrategyResponse,
  tokens: ICRC1[]
) {
  const tokenNames = strategy.pools
    .flatMap((p) => p.split("_"))
    .filter(function onlyUnique(value, index, array) {
      return array.indexOf(value) === index;
    });
  const logos = tokenNames.map((tN) => getTokenLogo(tN, tokens));
  return logos;
}

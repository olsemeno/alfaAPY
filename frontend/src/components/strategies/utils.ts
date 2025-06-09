import { ICRC1 } from "../../idl/icrc1_oracle";
import { Strategy } from "../../services";

export function getTokenLogo(symbol: string, tokens: ICRC1[]) {
  return tokens.find((token) => token.symbol === symbol)?.logo?.[0] ?? "";
}

export function getStrategyTokenLogos(
  strategy: Strategy,
  tokens: ICRC1[]
) {
  const tokenNames = strategy.pools
    .flatMap((p) => [p.token0.symbol, p.token1.symbol])
    .filter(function onlyUnique(value, index, array) {
      return array.indexOf(value) === index;
    });
  const logos = tokenNames.map((tN) => getTokenLogo(tN, tokens));
  return logos;
}

export type ProfitLevel = 'LOW' | 'MEDIUM' | 'HIGH' | 'Hot';

export function getProfitLevel(strategy: Strategy): ProfitLevel {
  // Example logic: mark id 1 as 'Hot', otherwise use TVL for profit level
  if (strategy.id === 1) return 'Hot';
  const tvl = Number((strategy.tvl));
  if (tvl > 1000000) return 'LOW';
  if (tvl > 100000) return 'MEDIUM';
  return 'HIGH';
}

export function getProfitColor(level: ProfitLevel): string {
  switch (level) {
    case 'LOW':
      return 'bg-green-100 text-green-800';
    case 'MEDIUM':
      return 'bg-yellow-100 text-yellow-800';
    case 'HIGH':
      return 'bg-red-100 text-red-800';
    case 'Hot':
      return 'bg-red-500 text-white';
    default:
      return 'bg-gray-100 text-gray-800';
  }
}

use types::exchanges::TokenInfo;
use types::exchange_id::ExchangeId;

use serde_json;
use hex;

pub fn generate_pool_id(token0: &TokenInfo, token1: &TokenInfo, provider: &ExchangeId) -> String {
    let keys = (token0, token1, provider);
    let json = serde_json::to_string(&keys).unwrap();
    hex::encode(json)
}

pub fn decode_pool_id(hex_str: &str) -> Option<(TokenInfo, TokenInfo, ExchangeId)> {
    let bytes = hex::decode(hex_str).ok()?;
    let json_str = std::str::from_utf8(&bytes).ok()?;
    serde_json::from_str(json_str).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use types::CanisterId;

    #[test]
    fn test_generate_and_decode_pool_id() {
        let token0 = TokenInfo {
            symbol: "PANDE".to_string(),
            ledger: CanisterId::from_text("druyg-tyaaa-aaaaq-aactq-cai").unwrap(),
        };
        let token1 = TokenInfo {
            symbol: "ICP".to_string(),
            ledger: CanisterId::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap(),
        };
        let provider = ExchangeId::KongSwap;
        let pool_id = generate_pool_id(&token0, &token1, &provider);

        assert_eq!(pool_id, "5b7b2273796d626f6c223a2250414e4445222c226c6564676572223a2264727579672d74796161612d61616161712d61616374712d636169227d2c7b2273796d626f6c223a22494350222c226c6564676572223a2272796a6c332d74796161612d61616161612d61616162612d636169227d2c224b6f6e6753776170225d");

        let (decoded_token0, decoded_token1, decoded_provider) =
        decode_pool_id(&pool_id).expect("Failed to decode pool id");

        assert_eq!(token0, decoded_token0);
        assert_eq!(token1, decoded_token1);
        assert_eq!(provider, decoded_provider);
    }
}

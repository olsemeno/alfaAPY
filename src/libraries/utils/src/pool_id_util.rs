use types::exchange_id::ExchangeId;
use types::CanisterId;

pub fn generate_pool_id(token0: &CanisterId, token1: &CanisterId, provider: &ExchangeId) -> String {
    format!("{}_{}_{}", provider, token0.to_text(), token1.to_text())
}

pub fn decode_pool_id(pool_id: &str) -> Option<(CanisterId, CanisterId, ExchangeId)> {
    let parts: Vec<&str> = pool_id.split('_').collect();

    if parts.len() != 3 {
        return None;
    }

    let provider = match parts[0] {
        "ICPSwap" => ExchangeId::ICPSwap,
        "Sonic" => ExchangeId::Sonic,
        "KongSwap" => ExchangeId::KongSwap,
        _ => return None,
    };
    let token0 = CanisterId::from_text(parts[1]).ok()?;
    let token1 = CanisterId::from_text(parts[2]).ok()?;

    Some((token0, token1, provider))
}

#[cfg(test)]
mod tests {
    use super::*;
    use types::CanisterId;

    #[test]
    fn test_generate_and_decode_pool_id() {
        let token0 = CanisterId::from_text("druyg-tyaaa-aaaaq-aactq-cai").unwrap();
        let token1 = CanisterId::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
        let provider = ExchangeId::KongSwap;
        let pool_id = generate_pool_id(&token0, &token1, &provider);

        assert_eq!(pool_id, "KongSwap_druyg-tyaaa-aaaaq-aactq-cai_ryjl3-tyaaa-aaaaa-aaaba-cai");

        let (decoded_token0, decoded_token1, decoded_provider) =
            decode_pool_id(&pool_id).expect("Failed to decode pool id");

        assert_eq!(token0, decoded_token0);
        assert_eq!(token1, decoded_token1);
        assert_eq!(provider, decoded_provider);
    }
}

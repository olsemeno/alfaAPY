use crate::exchange_id::ExchangeId;
use crate::CanisterId;

pub trait PoolTrait {
    fn get_id(&self) -> String;
    fn get_token0(&self) -> CanisterId;
    fn get_token1(&self) -> CanisterId;
    fn get_provider(&self) -> ExchangeId;
    fn new(id: String, token0: CanisterId, token1: CanisterId, provider: ExchangeId) -> Self;
    fn build(token0: CanisterId, token1: CanisterId, provider: ExchangeId) -> Self;
    fn is_same_pool(&self, compared_pool: &Self) -> bool;

    #[doc(hidden)]
    fn generate_pool_id(token0: &CanisterId, token1: &CanisterId, provider: &ExchangeId) -> String {
        format!("{}_{}_{}", provider, token0.to_text(), token1.to_text())
    }

    #[doc(hidden)]
    fn decode_pool_id(pool_id: &str) -> Option<(CanisterId, CanisterId, ExchangeId)> {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestPool {
        id: String,
        token0: CanisterId,
        token1: CanisterId,
        provider: ExchangeId,
    }

    impl PoolTrait for TestPool {
        fn get_id(&self) -> String { self.id.clone() }
        fn get_token0(&self) -> CanisterId { self.token0 }
        fn get_token1(&self) -> CanisterId { self.token1 }
        fn get_provider(&self) -> ExchangeId { self.provider }
        fn is_same_pool(&self, compared_pool: &Self) -> bool {
            let (token0, token1, provider) = Self::decode_pool_id(&compared_pool.id).unwrap();
            self.provider == provider && (
                (self.token0 == token0 && self.token1 == token1) ||
                (self.token0 == token1 && self.token1 == token0)
            )
        }
        fn new(id: String, token0: CanisterId, token1: CanisterId, provider: ExchangeId) -> Self {
            Self { id, token0, token1, provider }
        }
        fn build(token0: CanisterId, token1: CanisterId, provider: ExchangeId) -> Self {
            let id = Self::generate_pool_id(&token0, &token1, &provider);
            Self::new(id, token0, token1, provider)
        }
    }

    #[test]
    fn test_pool_id() {
        let token0 = CanisterId::from_text("druyg-tyaaa-aaaaq-aactq-cai").unwrap();
        let token1 = CanisterId::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
        let provider = ExchangeId::KongSwap;
        
        // Create a pool to test the functionality
        let pool = TestPool::build(token0, token1, provider);
        let pool_id = pool.get_id();

        assert_eq!(pool_id, "KongSwap_druyg-tyaaa-aaaaq-aactq-cai_ryjl3-tyaaa-aaaaa-aaaba-cai");

        // Test is_same_pool instead of direct decode
        let other_pool = TestPool::build(token0, token1, provider);
        assert!(pool.is_same_pool(&other_pool));
    }
}

use std::sync::Arc;

use utils::environment::Environment;

use crate::icpswap::{ICPSwapProvider, DefaultICPSwapProvider};
use crate::kongswap::{KongSwapProvider, DefaultKongSwapProvider};
use crate::mock::icpswap::MockICPSwapProvider;
use crate::mock::kongswap::MockKongSwapProvider;

#[derive(Clone)]
pub struct ProviderImpls {
    pub kongswap: Arc<dyn KongSwapProvider + Send + Sync>,
    pub icpswap: Arc<dyn ICPSwapProvider + Send + Sync>,
}

pub fn get_provider_impls(environment: Environment) -> ProviderImpls {
    ProviderImpls {
        kongswap: get_kongswap_provider_impl(environment),
        icpswap: get_icpswap_provider_impl(environment),
    }
}

fn get_kongswap_provider_impl(env: Environment) -> Arc<dyn KongSwapProvider + Send + Sync> {
    if env.should_use_mock_providers() {
        Arc::new(MockKongSwapProvider::new())
    } else {
        Arc::new(DefaultKongSwapProvider)
    }
}

fn get_icpswap_provider_impl(env: Environment) -> Arc<dyn ICPSwapProvider + Send + Sync> {
    if env.should_use_mock_providers() {
        Arc::new(MockICPSwapProvider::new())
    } else {
        Arc::new(DefaultICPSwapProvider)
    }
}

use std::sync::Arc;

use providers::{
    providers_factory::ProviderImpls,
    kongswap::DefaultKongSwapProvider,
    icpswap::DefaultICPSwapProvider,
    mock::kongswap::MockKongSwapProvider,
    mock::icpswap::MockICPSwapProvider,
};

use crate::repository::runtime_config_repo;

pub fn get_environment_provider_impls() -> ProviderImpls {
    let environment = runtime_config_repo::get_current_env();

    if environment.should_use_mock_providers() {
        ProviderImpls {
            kongswap: Arc::new(MockKongSwapProvider::new()),
            icpswap: Arc::new(MockICPSwapProvider::new()),
        }
    } else {
        ProviderImpls {
            kongswap: Arc::new(DefaultKongSwapProvider),
            icpswap: Arc::new(DefaultICPSwapProvider),
        }
    }
}

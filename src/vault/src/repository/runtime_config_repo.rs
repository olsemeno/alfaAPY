use std::cell::RefCell;
use candid::{CandidType, Deserialize};
use serde::Serialize;

use ::utils::environment::Environment;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct RuntimeConfig {
    pub environment: Environment,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            environment: Environment::Production,
        }
    }
}

thread_local! {
    static RUNTIME_CONFIG: RefCell<RuntimeConfig> = RefCell::new(RuntimeConfig::default());
}

pub fn get_runtime_config() -> RuntimeConfig {
    RUNTIME_CONFIG.with(|rc| rc.borrow().clone())
}

pub fn set_runtime_config(config: RuntimeConfig) {
    RUNTIME_CONFIG.with(|rc| {
        rc.borrow_mut();
        rc.replace(config);
    });
}

pub fn get_current_env() -> Environment {
    RUNTIME_CONFIG.with(|rc| rc.borrow().environment)
}

pub fn set_current_env(environment: Environment) {
    RUNTIME_CONFIG.with(|rc| {
        rc.borrow_mut().environment = environment;
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_env() {
        // Should start with Production environment by default
        assert_eq!(get_current_env(), Environment::Production);

        // Should be able to change to Test environment
        set_current_env(Environment::Test);
        assert_eq!(get_current_env(), Environment::Test);

        // Should be able to change back to Production
        set_current_env(Environment::Production);
        assert_eq!(get_current_env(), Environment::Production);
    }

    #[test]
    fn test_runtime_config() {
        // Should start with default config
        let config = get_runtime_config();
        assert_eq!(config.environment, Environment::Production);

        // Should be able to set new config
        let new_config = RuntimeConfig {
            environment: Environment::Test,
        };
        set_runtime_config(new_config.clone());
        assert_eq!(get_runtime_config().environment, Environment::Test);
    }
}

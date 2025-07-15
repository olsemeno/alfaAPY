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
    use ::utils::environment::Environment;

    mod get_current_env {
        use super::*;

        #[test]
        fn returns_production_by_default() {
            // Default environment is Production
            assert_eq!(get_current_env(), Environment::Production);
        }

        #[test]
        fn returns_updated_value_after_set_current_env() {
            let original = get_current_env();
            set_current_env(Environment::Test);
            assert_eq!(get_current_env(), Environment::Test);
            set_current_env(original);
        }
    }

    mod set_current_env {
        use super::*;

        #[test]
        fn sets_to_test_and_restores_back() {
            let original = get_current_env();
            set_current_env(Environment::Test);
            assert_eq!(get_current_env(), Environment::Test);
            set_current_env(original);
        }

        #[test]
        fn sets_back_to_production() {
            set_current_env(Environment::Production);
            assert_eq!(get_current_env(), Environment::Production);
        }
    }

    mod get_runtime_config {
        use super::*;

        #[test]
        fn returns_default_production_config() {
            let config = get_runtime_config();
            assert_eq!(config.environment, Environment::Production);
        }

        #[test]
        fn reflects_updated_config_after_set_runtime_config() {
            let original = get_runtime_config();

            let new_config = RuntimeConfig {
                environment: Environment::Test,
            };
            set_runtime_config(new_config.clone());

            let updated = get_runtime_config();
            assert_eq!(updated.environment, Environment::Test);

            set_runtime_config(original);
        }
    }

    mod set_runtime_config {
        use super::*;

        #[test]
        fn updates_runtime_config_environment() {
            let original = get_runtime_config();

            let test_config = RuntimeConfig {
                environment: Environment::Test,
            };
            set_runtime_config(test_config.clone());

            let result = get_runtime_config();
            assert_eq!(result.environment, Environment::Test);

            set_runtime_config(original);
        }

        #[test]
        fn can_set_back_to_production() {
            let production_config = RuntimeConfig {
                environment: Environment::Production,
            };
            set_runtime_config(production_config.clone());

            let result = get_runtime_config();
            assert_eq!(result.environment, Environment::Production);
        }
    }
}

use candid::{CandidType, Deserialize};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, CandidType, Serialize, Deserialize)]
pub enum Environment {
    Test,
    Production,
}

impl Environment {
    pub fn should_use_mock_providers(&self) -> bool {
        matches!(self, Environment::Test)
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "test" => Environment::Test,
            _ => Environment::Production,
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Environment::Production
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Environment::Test => write!(f, "test"),
            Environment::Production => write!(f, "production"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_mock_providers() {
        assert!(Environment::Test.should_use_mock_providers());
        assert!(!Environment::Production.should_use_mock_providers());
    }

    #[test]
    fn test_environment_from_str() {
        assert_eq!(Environment::from_str("test"), Environment::Test);
        assert_eq!(Environment::from_str("production"), Environment::Production);
        assert_eq!(Environment::from_str("unknown"), Environment::Production);
    }
}

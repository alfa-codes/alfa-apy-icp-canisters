use candid::{CandidType, Deserialize};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, CandidType, Serialize, Deserialize)]
pub enum Environment {
    Test,
    Dev,
    Staging,
    Production,
}

impl Environment {
    pub fn should_use_mock_services(&self) -> bool {
        matches!(self, Environment::Test)
    }

    pub fn is_test(&self) -> bool {
        matches!(self, Environment::Test)
    }

    pub fn is_dev(&self) -> bool {
        matches!(self, Environment::Dev)
    }

    pub fn is_staging(&self) -> bool {
        matches!(self, Environment::Staging)
    }

    pub fn is_production(&self) -> bool {
        matches!(self, Environment::Production)
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "test" => Environment::Test,
            "dev" => Environment::Dev,
            "staging" => Environment::Staging,
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
            Environment::Dev => write!(f, "dev"),
            Environment::Staging => write!(f, "staging"),
            Environment::Production => write!(f, "production"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_mock_services() {
        assert!(Environment::Test.should_use_mock_services());
        assert!(Environment::Dev.should_use_mock_services());
        assert!(Environment::Staging.should_use_mock_services());
        assert!(!Environment::Production.should_use_mock_services());
    }

    #[test]
    fn test_environment_from_str() {
        assert_eq!(Environment::from_str("test"), Environment::Test);
        assert_eq!(Environment::from_str("dev"), Environment::Dev);
        assert_eq!(Environment::from_str("staging"), Environment::Staging);
        assert_eq!(Environment::from_str("production"), Environment::Production);
    }
}

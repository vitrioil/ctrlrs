//! Configuration for the ctrl-r application

use crate::error::Error;
use crate::shell::ShellType;
use crate::Result;
use std::path::PathBuf;

/// Configuration for the ctrl-r application
#[derive(Debug, Clone)]
pub struct Config {
    /// The type of shell
    pub shell_type: ShellType,
    /// The path to the history file
    pub history_file: PathBuf,
}

impl Config {
    /// Create a new configuration
    ///
    /// If shell_type or history_file are not provided, they will be auto-detected.
    pub fn new(shell_type: Option<String>, history_file: Option<String>) -> Result<Self> {
        let shell_type = if let Some(shell) = shell_type {
            ShellType::from_name(&shell).ok_or_else(|| {
                Error::ShellDetection(format!("Unsupported shell type: {}", shell))
            })?
        } else {
            ShellType::detect()?
        };

        let history_file = if let Some(path) = history_file {
            PathBuf::from(path)
        } else {
            shell_type.default_history_path()?
        };

        Ok(Self {
            shell_type,
            history_file,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_config_with_explicit_values() {
        let config = Config::new(
            Some("bash".to_string()),
            Some("/tmp/test_history".to_string()),
        )
        .unwrap();

        assert_eq!(config.shell_type, ShellType::Bash);
        assert_eq!(config.history_file, PathBuf::from("/tmp/test_history"));
    }
}

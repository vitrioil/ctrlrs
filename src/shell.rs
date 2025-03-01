//! Shell detection and shell-specific operations

use crate::error::Error;
use crate::Result;
use dirs::home_dir;
use std::env;
use std::path::PathBuf;
use std::process::Command;

/// Supported shell types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellType {
    /// Bash shell
    Bash,
    /// Zsh shell
    Zsh,
    /// Fish shell
    Fish,
}

impl ShellType {
    /// Detect the current shell type
    pub fn detect() -> Result<Self> {
        // Try to get the shell from the SHELL environment variable
        if let Ok(shell) = env::var("SHELL") {
            if let Some(shell_type) = Self::from_path(&shell) {
                return Ok(shell_type);
            }
        }

        // Try to get the shell from the process tree
        if let Ok(output) = Command::new("ps").args(["-p", "$PPID", "-o", "comm="]).output() {
            if let Ok(shell) = String::from_utf8(output.stdout) {
                let shell = shell.trim();
                if let Some(shell_type) = Self::from_name(shell) {
                    return Ok(shell_type);
                }
            }
        }

        // Default to Bash if we can't detect the shell
        Ok(Self::Bash)
    }

    /// Get the shell type from a shell name
    pub fn from_name(name: &str) -> Option<Self> {
        let name = name.trim().to_lowercase();
        if name.contains("bash") {
            Some(Self::Bash)
        } else if name.contains("zsh") {
            Some(Self::Zsh)
        } else if name.contains("fish") {
            Some(Self::Fish)
        } else {
            None
        }
    }

    /// Get the shell type from a shell path
    pub fn from_path(path: &str) -> Option<Self> {
        let path = path.trim().to_lowercase();
        if path.ends_with("bash") {
            Some(Self::Bash)
        } else if path.ends_with("zsh") {
            Some(Self::Zsh)
        } else if path.ends_with("fish") {
            Some(Self::Fish)
        } else {
            None
        }
    }

    /// Get the default history file path for this shell type
    pub fn default_history_path(&self) -> Result<PathBuf> {
        let home = home_dir().ok_or_else(|| Error::Other("Could not determine home directory".to_string()))?;
        
        match self {
            Self::Bash => Ok(home.join(".bash_history")),
            Self::Zsh => Ok(home.join(".zsh_history")),
            Self::Fish => Ok(home.join(".local/share/fish/fish_history")),
        }
    }

    /// Get the history file format for this shell type
    pub fn history_format(&self) -> HistoryFormat {
        match self {
            Self::Bash => HistoryFormat::Plain,
            Self::Zsh => HistoryFormat::ZshExtended,
            Self::Fish => HistoryFormat::Fish,
        }
    }
}

/// History file formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistoryFormat {
    /// Plain text format (one command per line)
    Plain,
    /// Zsh extended format (with timestamps and other metadata)
    ZshExtended,
    /// Fish format (JSON-like)
    Fish,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_type_from_name() {
        assert_eq!(ShellType::from_name("bash"), Some(ShellType::Bash));
        assert_eq!(ShellType::from_name("zsh"), Some(ShellType::Zsh));
        assert_eq!(ShellType::from_name("fish"), Some(ShellType::Fish));
        assert_eq!(ShellType::from_name("unknown"), None);
    }

    #[test]
    fn test_shell_type_from_path() {
        assert_eq!(ShellType::from_path("/bin/bash"), Some(ShellType::Bash));
        assert_eq!(ShellType::from_path("/usr/bin/zsh"), Some(ShellType::Zsh));
        assert_eq!(ShellType::from_path("/usr/bin/fish"), Some(ShellType::Fish));
        assert_eq!(ShellType::from_path("/bin/unknown"), None);
    }

    #[test]
    fn test_default_history_path() {
        let home = home_dir().unwrap();
        
        assert_eq!(
            ShellType::Bash.default_history_path().unwrap(),
            home.join(".bash_history")
        );
        
        assert_eq!(
            ShellType::Zsh.default_history_path().unwrap(),
            home.join(".zsh_history")
        );
        
        assert_eq!(
            ShellType::Fish.default_history_path().unwrap(),
            home.join(".local/share/fish/fish_history")
        );
    }
}

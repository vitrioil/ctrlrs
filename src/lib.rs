//! Enhanced Ctrl-R for shell history with n-dimensional search
//!
//! This library provides functionality for searching shell history
//! with an n-dimensional approach, allowing for multiple levels of nested filtering.

pub mod app;
pub mod config;
pub mod history;
pub mod shell;
pub mod ui;

/// Error types for the ctrl-r application
pub mod error {
    use thiserror::Error;

    /// Errors that can occur in the ctrl-r application
    #[derive(Error, Debug)]
    pub enum Error {
        /// Error when detecting the shell
        #[error("Failed to detect shell: {0}")]
        ShellDetection(String),

        /// Error when reading history file
        #[error("Failed to read history file: {0}")]
        HistoryRead(String),

        /// Error when parsing history
        #[error("Failed to parse history: {0}")]
        HistoryParse(String),

        /// Error with the terminal UI
        #[error("Terminal UI error: {0}")]
        Ui(String),

        /// IO error
        #[error("IO error: {0}")]
        Io(#[from] std::io::Error),

        /// Other errors
        #[error("Other error: {0}")]
        Other(String),
    }
}

/// Result type for the ctrl-r application
pub type Result<T> = std::result::Result<T, error::Error>;

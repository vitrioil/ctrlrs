//! Application state and logic

use crate::config::Config;
use crate::history::{HistoryEntry, HistoryManager};
use crate::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Result type for app operations
pub type AppResult<T> = crate::Result<T>;

/// Application state
#[derive(Debug)]
pub struct App {
    /// Application configuration
    config: Config,
    /// History manager
    history_manager: HistoryManager,
    /// Filters for each dimension
    filters: Vec<String>,
    /// Current filter dimension (0-based index)
    current_dimension: usize,
    /// Maximum number of dimensions
    max_dimensions: usize,
    /// Filtered history entries
    filtered_entries: Vec<HistoryEntry>,
    /// Selected entry index
    selected_index: usize,
    /// Whether the application should exit
    should_quit: bool,
    /// Selected command to return
    selected_command: Option<String>,
}

impl App {
    /// Create a new application instance
    pub fn new(config: Config) -> Result<Self> {
        let history_manager = HistoryManager::new(&config)?;
        
        // Initialize with 5 empty filters
        let mut filters = Vec::with_capacity(5);
        for _ in 0..5 {
            filters.push(String::new());
        }
        
        Ok(Self {
            config,
            history_manager,
            filters,
            current_dimension: 0, // 0-based index (first dimension is 0)
            max_dimensions: 5,    // Maximum of 5 dimensions
            filtered_entries: Vec::new(),
            selected_index: 0,
            should_quit: false,
            selected_command: None,
        })
    }

    /// Update filters and filtered entries
    pub fn update_filters(&mut self) -> AppResult<()> {
        // Apply all active filters
        let active_filters: Vec<&str> = self.filters.iter()
            .take(self.current_dimension + 1) // Only use filters up to current dimension
            .map(|s| s.as_str())
            .collect();
        
        self.filtered_entries = self.history_manager.filter_multiple(&active_filters)?;
        
        // Reset selected index if it's out of bounds
        if !self.filtered_entries.is_empty() && self.selected_index >= self.filtered_entries.len() {
            self.selected_index = self.filtered_entries.len() - 1;
        }
        
        Ok(())
    }

    /// Handle key events
    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> AppResult<()> {
        match key_event.code {
            // Quit
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.should_quit = true;
            }
            KeyCode::Esc => {
                self.should_quit = true;
            }
            
            // Navigation
            KeyCode::Up => {
                if !self.filtered_entries.is_empty() {
                    self.selected_index = self.selected_index.saturating_sub(1);
                }
            }
            KeyCode::Down => {
                if !self.filtered_entries.is_empty() {
                    self.selected_index = (self.selected_index + 1).min(self.filtered_entries.len() - 1);
                }
            }
            
            // Selection
            KeyCode::Enter => {
                if !self.filtered_entries.is_empty() {
                    self.selected_command = Some(self.filtered_entries[self.selected_index].command.clone());
                    self.should_quit = true;
                }
            }
            
            // Switch to next dimension
            KeyCode::Char('r') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                // Only move to next dimension if current filter is not empty and we haven't reached max
                if !self.filters[self.current_dimension].is_empty() && 
                   self.current_dimension < self.max_dimensions - 1 {
                    self.current_dimension += 1;
                }
            }
            
            // Text input
            KeyCode::Char(c) => {
                self.filters[self.current_dimension].push(c);
                self.update_filters()?;
            }
            
            KeyCode::Backspace => {
                if !self.filters[self.current_dimension].is_empty() {
                    self.filters[self.current_dimension].pop();
                    self.update_filters()?;
                } else if self.current_dimension > 0 {
                    // Go back to previous dimension if current filter is empty
                    self.current_dimension -= 1;
                }
            }
            
            _ => {}
        }
        
        Ok(())
    }

    /// Get the filter for a specific dimension
    pub fn filter(&self, dimension: usize) -> &str {
        &self.filters[dimension]
    }

    /// Get the current filter dimension (0-based)
    pub fn current_dimension(&self) -> usize {
        self.current_dimension
    }
    
    /// Get the maximum number of dimensions
    pub fn max_dimensions(&self) -> usize {
        self.max_dimensions
    }

    /// Get the filtered history entries
    pub fn filtered_entries(&self) -> &[HistoryEntry] {
        &self.filtered_entries
    }

    /// Get the selected index
    pub fn selected_index(&self) -> usize {
        self.selected_index
    }

    /// Check if the application should quit
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    /// Get the selected command
    pub fn selected_command(&self) -> Option<&str> {
        self.selected_command.as_deref()
    }
}



//! History file parsing and filtering

use crate::config::Config;
use crate::error::Error;
use crate::shell::{HistoryFormat, ShellType};
use crate::Result;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// A history entry
#[derive(Debug, Clone)]
pub struct HistoryEntry {
    /// The command
    pub command: String,
    /// The timestamp (if available)
    pub timestamp: Option<u64>,
    /// The original line from the history file
    pub original_line: String,
}

/// History manager
#[derive(Debug, Clone)]
pub struct HistoryManager {
    /// The history entries
    entries: Vec<HistoryEntry>,
    /// The shell type
    shell_type: ShellType,
}

impl HistoryManager {
    /// Create a new history manager
    pub fn new(config: &Config) -> Result<Self> {
        let entries = Self::read_history_file(&config.history_file, config.shell_type)?;
        
        Ok(Self {
            entries,
            shell_type: config.shell_type,
        })
    }

    /// Read history entries from a file
    fn read_history_file(path: &Path, shell_type: ShellType) -> Result<Vec<HistoryEntry>> {
        let file = File::open(path).map_err(|e| {
            Error::HistoryRead(format!("Failed to open history file: {}", e))
        })?;
        
        let format = shell_type.history_format();
        let mut entries = Vec::new();
        
        // Process line by line for all formats
        let reader = BufReader::new(file);
        let mut current_command = String::new();
        let mut current_timestamp: Option<u64> = None;
        let timestamp_regex = Regex::new(r"^: (\d+):\d+;(.*)$").unwrap();
        
        // Read the file line by line, skipping lines with invalid UTF-8
        for line_result in reader.lines() {
            match line_result {
                Ok(line) => {
                    if format == HistoryFormat::ZshExtended {
                        // Check if this is a new command (starts with timestamp)
                        if let Some(captures) = timestamp_regex.captures(&line) {
                            // If we have a previous command, add it as an entry
                            if !current_command.is_empty() {
                                if let Some(entry) = Self::create_entry(&current_command, current_timestamp, format) {
                                    entries.push(entry);
                                }
                                current_command.clear();
                            }
                            
                            // Extract timestamp and command
                            if let Some(ts_str) = captures.get(1) {
                                if let Ok(ts) = ts_str.as_str().parse::<u64>() {
                                    current_timestamp = Some(ts);
                                }
                            }
                            
                            if let Some(cmd) = captures.get(2) {
                                current_command = cmd.as_str().to_string();
                            }
                        } else {
                            // This is a continuation of the previous command
                            if !current_command.is_empty() {
                                current_command.push('\n');
                                current_command.push_str(&line);
                            }
                        }
                    } else {
                        // For other formats, each line is a separate command
                        if let Some(entry) = Self::parse_history_line(&line, format) {
                            entries.push(entry);
                        }
                    }
                },
                Err(e) => {
                    // Log the error but continue processing
                    log::warn!("Skipping line with invalid UTF-8: {}", e);
                    continue;
                }
            }
        }
        
        // Add the last command if there is one
        if format == HistoryFormat::ZshExtended && !current_command.is_empty() {
            if let Some(entry) = Self::create_entry(&current_command, current_timestamp, format) {
                entries.push(entry);
            }
        }
        
        // Sort entries by timestamp (if available) or just keep the order
        entries.sort_by(|a, b| {
            match (a.timestamp, b.timestamp) {
                (Some(a_ts), Some(b_ts)) => b_ts.cmp(&a_ts), // Reverse order (newest first)
                (Some(_), None) => std::cmp::Ordering::Less, // Entries with timestamps come first
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => std::cmp::Ordering::Equal,
            }
        });
        
        // Deduplicate entries, keeping only the first occurrence of each command
        // (which will be the most recent due to the sorting above)
        let mut unique_entries = Vec::new();
        let mut seen_commands = std::collections::HashSet::new();
        
        for entry in entries {
            if seen_commands.insert(entry.command.clone()) {
                unique_entries.push(entry);
            }
        }
        
        Ok(unique_entries)
    }

    /// Create a history entry from a command and timestamp
    fn create_entry(command: &str, timestamp: Option<u64>, _format: HistoryFormat) -> Option<HistoryEntry> {
        if command.trim().is_empty() {
            return None;
        }
        
        // Convert multi-line commands to single line
        let mut processed_command = command.replace('\n', " ");
        // Handle both single and double backslashes (zsh stores backslashes as double backslashes)
        processed_command = processed_command.replace(" \\\\", " ").replace(" \\", " ");
        
        Some(HistoryEntry {
            command: processed_command,
            timestamp,
            original_line: command.to_string(),
        })
    }

    /// Parse a history line
    fn parse_history_line(line: &str, format: HistoryFormat) -> Option<HistoryEntry> {
        if line.trim().is_empty() {
            return None;
        }
        
        match format {
            HistoryFormat::Plain => {
                // Simple format: just the command
                // Convert multi-line commands to single line
                // First, join all lines by replacing newlines with spaces
                let mut command = line.replace('\n', " ");
                // Then, remove any trailing backslashes that were used for line continuation
                // Handle both single and double backslashes
                command = command.replace(" \\\\", " ").replace(" \\", " ");
                Some(HistoryEntry {
                    command,
                    timestamp: None,
                    original_line: line.to_string(),
                })
            }
            HistoryFormat::ZshExtended => {
                // Zsh format: ": timestamp:0;command"
                let re = Regex::new(r"^: (\d+):\d+;(.*)$").ok()?;
                if let Some(captures) = re.captures(line) {
                    let timestamp = captures.get(1)?.as_str().parse::<u64>().ok()?;
                    let mut command = captures.get(2)?.as_str().replace('\n', " ");
                    // Handle both single and double backslashes (zsh stores backslashes as double backslashes)
                    command = command.replace(" \\\\", " ").replace(" \\", " ");
                    
                    Some(HistoryEntry {
                        command,
                        timestamp: Some(timestamp),
                        original_line: line.to_string(),
                    })
                } else {
                    // Fall back to treating it as a plain command
                    // Convert multi-line commands to single line
                    let mut command = line.replace('\n', " ");
                    command = command.replace(" \\\\", " ").replace(" \\", " ");
                    Some(HistoryEntry {
                        command,
                        timestamp: None,
                        original_line: line.to_string(),
                    })
                }
            }
            HistoryFormat::Fish => {
                // Fish format is complex JSON-like, simplified here
                // Just extract command part after "cmd:"
                if let Some(cmd_start) = line.find("cmd:") {
                    let cmd_part = &line[cmd_start + 4..];
                    if let Some(cmd_end) = cmd_part.find("when:") {
                        let mut command = cmd_part[..cmd_end].trim().trim_matches('"').replace('\n', " ");
                        command = command.replace(" \\\\", " ").replace(" \\", " ");
                        
                        // Try to extract timestamp
                        let timestamp = if let Some(when_start) = line.find("when:") {
                            let when_part = &line[when_start + 5..];
                            if let Some(when_end) = when_part.find(|c| c == ' ' || c == '\n') {
                                when_part[..when_end].trim().parse::<u64>().ok()
                            } else {
                                None
                            }
                        } else {
                            None
                        };
                        
                        Some(HistoryEntry {
                            command,
                            timestamp,
                            original_line: line.to_string(),
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        }
    }

    /// Filter history entries with multiple filters
    pub fn filter_multiple(&self, filters: &[&str]) -> Result<Vec<HistoryEntry>> {
        // If no filters or first filter is empty, return all entries
        if filters.is_empty() || filters[0].is_empty() {
            return Ok(self.entries.clone());
        }
        
        // Start with all entries
        let mut filtered = self.entries.clone();
        
        // Apply each filter sequentially
        for filter in filters {
            if filter.is_empty() {
                continue; // Skip empty filters
            }
            
            let re = Regex::new(&format!("(?i){}", regex::escape(filter)))
                .map_err(|e| Error::Other(format!("Invalid regex: {}", e)))?;
            
            filtered = filtered.into_iter()
                .filter(|entry| re.is_match(&entry.command))
                .collect();
        }
        
        Ok(filtered)
    }
    
    /// Filter history entries (legacy method for backward compatibility)
    pub fn filter(&self, first_filter: &str, second_filter: &str) -> Result<Vec<HistoryEntry>> {
        let filters: Vec<&str> = if second_filter.is_empty() {
            vec![first_filter]
        } else {
            vec![first_filter, second_filter]
        };
        
        self.filter_multiple(&filters)
    }

    /// Get all history entries
    pub fn entries(&self) -> &[HistoryEntry] {
        &self.entries
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_parse_bash_history() {
        let line = "ls -la";
        let entry = HistoryManager::parse_history_line(line, HistoryFormat::Plain).unwrap();
        
        assert_eq!(entry.command, "ls -la");
        assert_eq!(entry.timestamp, None);
        assert_eq!(entry.original_line, "ls -la");
    }

    #[test]
    fn test_parse_zsh_history() {
        let line = ": 1738093190:0;ls -la";
        let entry = HistoryManager::parse_history_line(line, HistoryFormat::ZshExtended).unwrap();
        
        assert_eq!(entry.command, "ls -la");
        assert_eq!(entry.timestamp, Some(1738093190));
        assert_eq!(entry.original_line, ": 1738093190:0;ls -la");
    }

    #[test]
    fn test_filter_history() {
        let entries = vec![
            HistoryEntry {
                command: "ls -la".to_string(),
                timestamp: None,
                original_line: "ls -la".to_string(),
            },
            HistoryEntry {
                command: "cd /tmp".to_string(),
                timestamp: None,
                original_line: "cd /tmp".to_string(),
            },
            HistoryEntry {
                command: "ls -l /tmp".to_string(),
                timestamp: None,
                original_line: "ls -l /tmp".to_string(),
            },
        ];
        
        let manager = HistoryManager {
            entries,
            shell_type: ShellType::Bash,
        };
        
        // Test first filter
        let filtered = manager.filter("ls", "").unwrap();
        assert_eq!(filtered.len(), 2);
        assert_eq!(filtered[0].command, "ls -la");
        assert_eq!(filtered[1].command, "ls -l /tmp");
        
        // Test second filter
        let filtered = manager.filter("ls", "tmp").unwrap();
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].command, "ls -l /tmp");
    }
}

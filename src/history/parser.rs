use anyhow::{Context, Result};
use chrono::{DateTime, Local, TimeZone};
use dirs::home_dir;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct HistoryEntry {
    pub timestamp: DateTime<Local>,
    pub command: String,
}

pub struct HistoryParser {
    history_file: PathBuf,
}

impl HistoryParser {
    pub fn new() -> Result<Self> {
        let home = home_dir().context("Could not find home directory")?;
        let history_file = home.join(".zsh_history");
        
        Ok(Self { history_file })
    }

    pub fn read_history(&self) -> Result<Vec<HistoryEntry>> {
        let file = File::open(&self.history_file)
            .context("Failed to open history file")?;
        
        // Use a BufReader with explicit UTF-8 decoding
        let reader = BufReader::new(file);
        let mut entries = Vec::new();

        for line in reader.split(b'\n') {
            let line = line?;
            // Convert bytes to string, skipping invalid UTF-8 sequences
            let line_str = String::from_utf8_lossy(&line);
            if let Some(entry) = self.parse_line(&line_str) {
                entries.push(entry);
            }
        }

        Ok(entries)
    }

    fn parse_line(&self, line: &str) -> Option<HistoryEntry> {
        // ZSH history format: ": timestamp:0;command"
        if let Some(content) = line.strip_prefix(": ") {
            if let Some((timestamp_str, command)) = content.split_once(";") {
                if let Some((timestamp, _)) = timestamp_str.split_once(":") {
                    if let Ok(timestamp) = timestamp.parse::<i64>() {
                        return Some(HistoryEntry {
                            timestamp: Local.timestamp_opt(timestamp, 0)
                                .single()
                                .unwrap_or_else(|| Local::now()),
                            command: command.to_string(),
                        });
                    }
                }
            }
        }
        None
    }

    pub fn get_entries_for_date(&self, date: &str) -> Result<Vec<HistoryEntry>> {
        let all_entries = self.read_history()?;
        let target_date = chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d")?;
        
        Ok(all_entries
            .into_iter()
            .filter(|entry| entry.timestamp.date_naive() == target_date)
            .collect())
    }
} 
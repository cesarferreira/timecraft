use std::collections::HashMap;
use crate::history::parser::HistoryEntry;
use anyhow::Result;

pub struct StatsAnalyzer {
    entries: Vec<HistoryEntry>,
}

impl StatsAnalyzer {
    pub fn new(entries: Vec<HistoryEntry>) -> Self {
        Self { entries }
    }

    pub fn get_top_commands(&self, limit: usize) -> Vec<(String, usize)> {
        let mut command_counts: HashMap<String, usize> = HashMap::new();
        
        for entry in &self.entries {
            let command = entry.command.split_whitespace()
                .next()
                .unwrap_or("")
                .to_string();
            
            if !command.is_empty() {
                *command_counts.entry(command).or_insert(0) += 1;
            }
        }

        let mut counts: Vec<(String, usize)> = command_counts.into_iter().collect();
        counts.sort_by(|a, b| b.1.cmp(&a.1));
        counts.truncate(limit);
        
        counts
    }

    pub fn suggest_aliases(&self) -> Vec<(String, String)> {
        let mut suggestions = Vec::new();
        let counts = self.get_top_commands(10);
        
        for (cmd, count) in counts {
            if count > 10 {
                match cmd.as_str() {
                    "git" => {
                        suggestions.push(("g".to_string(), "git".to_string()));
                        suggestions.push(("gp".to_string(), "git push".to_string()));
                        suggestions.push(("gl".to_string(), "git pull".to_string()));
                    }
                    "docker" => {
                        suggestions.push(("d".to_string(), "docker".to_string()));
                        suggestions.push(("dc".to_string(), "docker-compose".to_string()));
                    }
                    "npm" => {
                        suggestions.push(("nr".to_string(), "npm run".to_string()));
                        suggestions.push(("ni".to_string(), "npm install".to_string()));
                    }
                    _ => {}
                }
            }
        }
        
        suggestions
    }

    pub fn get_daily_stats(&self) -> Result<DailyStats> {
        let total_commands = self.entries.len();
        let unique_commands: usize = self.entries
            .iter()
            .map(|e| e.command.split_whitespace().next().unwrap_or(""))
            .collect::<std::collections::HashSet<_>>()
            .len();

        Ok(DailyStats {
            total_commands,
            unique_commands,
            top_tools: self.get_top_commands(3),
        })
    }
}

pub struct DailyStats {
    pub total_commands: usize,
    pub unique_commands: usize,
    pub top_tools: Vec<(String, usize)>,
} 
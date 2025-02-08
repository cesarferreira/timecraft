use crate::history::parser::HistoryEntry;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use std::collections::HashMap;

pub struct AuditAnalyzer {
    entries: Vec<HistoryEntry>,
}

#[derive(Debug)]
pub struct TypoSuggestion {
    pub typed: String,
    pub meant: String,
    pub count: usize,
}

#[derive(Debug)]
pub struct DangerousCommand {
    pub command: String,
    pub occurrences: usize,
    pub risk_level: RiskLevel,
    pub suggestion: String,
}

#[derive(Debug)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
}

impl AuditAnalyzer {
    pub fn new(entries: Vec<HistoryEntry>) -> Self {
        Self { entries }
    }

    pub fn find_typos(&self) -> Vec<TypoSuggestion> {
        let common_commands = vec![
            "ls", "cd", "git", "docker", "npm", "yarn", "cargo",
            "python", "pip", "node", "vim", "code",
        ];
        let matcher = SkimMatcherV2::default();
        let mut typo_counts: HashMap<String, (String, usize)> = HashMap::new();

        for entry in &self.entries {
            let command = entry.command.split_whitespace().next().unwrap_or("").to_string();
            if command.len() >= 2 {
                for &correct in common_commands.iter() {
                    if command != correct && matcher.fuzzy_match(&command, correct).is_some() {
                        let entry = typo_counts.entry(command.clone()).or_insert((correct.to_string(), 0));
                        entry.1 += 1;
                    }
                }
            }
        }

        typo_counts
            .into_iter()
            .map(|(typed, (meant, count))| TypoSuggestion { typed, meant, count })
            .collect()
    }

    pub fn find_dangerous_commands(&self) -> Vec<DangerousCommand> {
        let mut dangerous = Vec::new();
        let mut command_counts: HashMap<String, usize> = HashMap::new();

        for entry in &self.entries {
            let command = entry.command.to_string();
            *command_counts.entry(command).or_insert(0) += 1;
        }

        for (command, count) in command_counts {
            if command.contains("rm -rf") {
                dangerous.push(DangerousCommand {
                    command: command.clone(),
                    occurrences: count,
                    risk_level: RiskLevel::High,
                    suggestion: "Use 'trash' command or add '-i' flag for interactive deletion".to_string(),
                });
            } else if command.contains("sudo") && !command.contains("apt") && !command.contains("brew") {
                dangerous.push(DangerousCommand {
                    command,
                    occurrences: count,
                    risk_level: RiskLevel::Medium,
                    suggestion: "Consider using 'sudo -E' or creating a proper service".to_string(),
                });
            } else if command.contains("> /dev/null 2>&1") {
                dangerous.push(DangerousCommand {
                    command,
                    occurrences: count,
                    risk_level: RiskLevel::Low,
                    suggestion: "Consider proper logging instead of discarding output".to_string(),
                });
            }
        }

        dangerous
    }
} 
use crate::history::parser::HistoryEntry;
use chrono::{DateTime, Duration, Local, Timelike};
use rand::seq::SliceRandom;
use std::collections::HashMap;

pub struct FunAnalyzer {
    entries: Vec<HistoryEntry>,
}

#[derive(Debug)]
pub struct FunFact {
    pub fact: String,
    pub emoji: String,
}

impl FunAnalyzer {
    pub fn new(entries: Vec<HistoryEntry>) -> Self {
        Self { entries }
    }

    pub fn generate_fun_facts(&self) -> Vec<FunFact> {
        let mut facts = Vec::new();
        
        // Longest coding session
        if let Some(session) = self.find_longest_session() {
            facts.push(FunFact {
                fact: format!("Your longest coding session was {} hours {} minutes",
                    session.num_hours(),
                    session.num_minutes() % 60),
                emoji: "🏃".to_string(),
            });
        }

        // Most productive hour
        if let Some((hour, count)) = self.most_productive_hour() {
            facts.push(FunFact {
                fact: format!("You're most productive at {}:00 with {} commands", hour, count),
                emoji: "⏰".to_string(),
            });
        }

        // Command variety
        let variety = self.command_variety();
        facts.push(FunFact {
            fact: format!("You've used {} unique commands out of {} total executions",
                variety.unique_commands,
                variety.total_commands),
            emoji: "🎭".to_string(),
        });

        facts
    }

    pub fn generate_roast(&self) -> String {
        let roasts = vec![
            "I see you're still typing 'gut status' instead of 'git status'. Maybe it's time for a typing course? 😏",
            "Wow, another 'sudo !!' - planning ahead isn't your strong suit, is it? 😅",
            "You've used 'vim' and immediately ':q' so many times. VS Code is waiting for you... 🎯",
            "The number of times you've run 'npm install' without '--save' is concerning. 📦",
            "Your command history suggests you're in a very committed relationship with Stack Overflow. 💑",
        ];

        roasts.choose(&mut rand::thread_rng())
            .unwrap_or(&"You're doing great! (I couldn't find anything to roast) 😇")
            .to_string()
    }

    fn find_longest_session(&self) -> Option<Duration> {
        if self.entries.is_empty() {
            return None;
        }

        let mut longest = Duration::zero();
        let mut current = Duration::zero();
        let mut last_time: Option<DateTime<Local>> = None;

        for entry in &self.entries {
            if let Some(last) = last_time {
                let gap = entry.timestamp - last;
                if gap < Duration::hours(1) {
                    current = current + gap;
                } else {
                    if current > longest {
                        longest = current;
                    }
                    current = Duration::zero();
                }
            }
            last_time = Some(entry.timestamp);
        }

        Some(longest)
    }

    fn most_productive_hour(&self) -> Option<(u32, usize)> {
        let mut hour_counts: HashMap<u32, usize> = HashMap::new();
        
        for entry in &self.entries {
            let hour = entry.timestamp.hour();
            *hour_counts.entry(hour).or_insert(0) += 1;
        }

        hour_counts.into_iter()
            .max_by_key(|&(_, count)| count)
    }

    fn command_variety(&self) -> CommandVariety {
        let total_commands = self.entries.len();
        let unique_commands = self.entries.iter()
            .map(|e| e.command.split_whitespace().next().unwrap_or(""))
            .collect::<std::collections::HashSet<_>>()
            .len();

        CommandVariety {
            total_commands,
            unique_commands,
        }
    }
}

struct CommandVariety {
    total_commands: usize,
    unique_commands: usize,
} 
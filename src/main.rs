use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use timecraft::{AuditAnalyzer, FunAnalyzer, HistoryParser, StatsAnalyzer};

#[derive(Parser)]
#[command(name = "timecraft")]
#[command(about = "Your Terminal's Time Machine - Analyze and optimize your shell history", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show usage statistics and patterns
    Stats {
        /// Show top most used commands
        #[arg(long)]
        top_commands: bool,
        /// Show daily statistics
        #[arg(long)]
        daily: bool,
    },
    /// Audit your command history for patterns and issues
    Audit {
        /// Check for common typos
        #[arg(long)]
        typos: bool,
        /// Check for potentially dangerous commands
        #[arg(long)]
        danger: bool,
    },
    /// Generate optimization suggestions
    Optimize {
        /// Automatically add suggested aliases to shell config
        #[arg(long)]
        auto: bool,
    },
    /// Replay a past terminal session
    Replay {
        /// Date to replay (YYYY-MM-DD)
        date: String,
    },
    /// Show fun facts about your command usage
    Funfacts,
    /// Get a humorous roast about your command habits
    Roast,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let parser = HistoryParser::new()?;
    let history = parser.read_history()?;

    match cli.command {
        Commands::Stats { top_commands, daily } => {
            let analyzer = StatsAnalyzer::new(history);
            
            if top_commands {
                println!("{}", "🔥 Your top commands:".green());
                for (idx, (cmd, count)) in analyzer.get_top_commands(5).iter().enumerate() {
                    println!("{}. {} ({}x)", idx + 1, cmd.to_string().yellow(), count);
                }
            }

            if daily {
                let stats = analyzer.get_daily_stats()?;
                println!("\n📊 Daily Statistics:");
                println!("Total commands: {}", stats.total_commands);
                println!("Unique commands: {}", stats.unique_commands);
                println!("\nTop tools:");
                for (tool, count) in stats.top_tools {
                    println!("- {} ({}x)", tool.cyan(), count);
                }
            }
        }

        Commands::Audit { typos, danger } => {
            let analyzer = AuditAnalyzer::new(history);
            
            if typos {
                println!("{}", "🔍 Common typos found:".yellow());
                for typo in analyzer.find_typos() {
                    println!("You typed '{}' instead of '{}' {}x",
                        typo.typed.to_string().red(),
                        typo.meant.to_string().green(),
                        typo.count);
                }
            }

            if danger {
                println!("{}", "\n🚨 Potentially dangerous commands:".red());
                for cmd in analyzer.find_dangerous_commands() {
                    println!("Command: {} ({}x)", cmd.command.to_string().red(), cmd.occurrences);
                    println!("Suggestion: {}\n", cmd.suggestion.to_string().yellow());
                }
            }
        }

        Commands::Optimize { auto } => {
            let analyzer = StatsAnalyzer::new(history);
            println!("{}", "⚡ Suggested aliases:".blue());
            
            for (alias, command) in analyzer.suggest_aliases() {
                println!("alias {}='{}'", alias.to_string().green(), command);
            }

            if auto {
                println!("{}", "\nTo automatically add these aliases, run:".yellow());
                println!("timecraft optimize | grep alias >> ~/.zshrc");
            }
        }

        Commands::Replay { date } => {
            let entries = parser.get_entries_for_date(&date)?;
            println!("⏪ Replaying commands from {}:", date.cyan());
            
            for entry in entries {
                println!("{} {}", "→".green(), entry.command);
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
        }

        Commands::Funfacts => {
            let analyzer = FunAnalyzer::new(history);
            println!("{}", "📊 Fun Facts about your terminal usage:".magenta());
            
            for fact in analyzer.generate_fun_facts() {
                println!("{} {}", fact.emoji, fact.fact);
            }
        }

        Commands::Roast => {
            let analyzer = FunAnalyzer::new(history);
            println!("{}", "🔥 Roast incoming:".red());
            println!("{}", analyzer.generate_roast());
        }
    }

    Ok(())
}

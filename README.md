# TimeCraft: Your Terminal's Time Machine 🕰️

Unearth patterns, fix mistakes, and laugh at your past self—all by mining your command history.

## Features 🌟

- 📊 **Command Statistics**: Analyze your most used commands and daily patterns
- 🔍 **Typo Detection**: Find common mistakes in your typing
- ⚡ **Alias Suggestions**: Get smart suggestions for shortcuts
- 🚨 **Safety Checks**: Identify potentially dangerous command patterns
- ⏪ **Session Replay**: Relive your past terminal sessions
- 😄 **Fun Facts & Roasts**: Get entertaining insights about your command-line habits

## Installation 🚀

```bash
# Using cargo
cargo install timecraft # TODO: actually publish it

# Or build from source
git clone https://github.com/cesarferreira/timecraft.git
cd timecraft
cargo build --release
```

## Usage 💻

```bash
# Show your most used commands
timecraft stats --top-commands

# Get daily statistics
timecraft stats --daily

# Check for typos
timecraft audit --typos

# Find dangerous commands
timecraft audit --danger

# Get alias suggestions
timecraft optimize

# Automatically add aliases to your shell
timecraft optimize --auto

# Replay a specific day's commands
timecraft replay 2024-03-14

# Get fun facts about your usage
timecraft funfacts

# Get roasted about your habits
timecraft roast
```

## Examples 📝

```bash
$ timecraft stats --top-commands
🔥 Your top commands:
1. git push (127x)
2. npm run dev (89x)
3. cargo test (42x)

$ timecraft funfacts
📊 Fun Facts about your terminal usage:
🏃 Your longest coding session was 4 hours 37 minutes
⏰ You're most productive at 15:00 with 42 commands
🎭 You've used 89 unique commands out of 517 total executions

$ timecraft roast
🔥 Roast incoming:
I see you're still typing 'gut status' instead of 'git status'.
Maybe it's time for a typing course? 😏
```

## Requirements 📋

- Rust 1.70 or higher
- ZSH shell (support for other shells coming soon)
- Unix-like operating system (Linux, macOS)

## Contributing 🤝

Contributions are welcome! Please feel free to submit a Pull Request.

## License 📄

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments 🙏

- Inspired by the countless hours spent in terminals
- Built with Rust 🦀 and love ❤️ 
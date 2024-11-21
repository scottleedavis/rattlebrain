# Rocket League Replay Analyzer

A command-line tool that analyzes Rocket League replay files using the Claude AI API. This tool extracts meaningful insights, tactical analysis, and performance metrics from your replays.

## Features

- Parse Rocket League replay files using the `boxcars` crate
- Extract detailed game statistics and events
- Generate AI-powered analysis using Claude API
- Provide tactical insights and improvement suggestions

## Prerequisites

- Rust (latest stable version)
- Claude API key
- Rocket League replay files

## Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/rocket-league-analyzer.git
cd rocket-league-analyzer
```

2. Create a `.env` file in the project root:
```bash
CLAUDE_API_KEY=your-api-key-here
```

3. Build the project:
```bash
cargo build --release
```

## Usage

1. Basic analysis:
```bash
cargo run -- analyze path/to/replay.replay
```

2. Detailed analysis with specific focus:
```bash
cargo run -- analyze --focus tactical path/to/replay.replay
```

## Configuration

The analyzer can be configured through command-line arguments or environment variables:

- `CLAUDE_API_KEY`: Your Claude API key (required)
- `RL_ANALYZER_MODEL`: Claude model to use (default: "claude-3-sonnet-20240229")
- `RL_ANALYZER_LOG_LEVEL`: Log level (default: "info")

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [boxcars](https://crates.io/crates/boxcars) - Rocket League replay parser
- [Anthropic](https://anthropic.com) - Claude AI API
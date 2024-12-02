# rocket-league-ai-analysis

A command-line fully Rust-based replay analysis tool that analyzes Rocket League replay files using one or more AI services.  I utilizes smart environment variable detection for API keys to dynamically select AI interfaces (e.g., OpenAI, Claude).
It provides nified reporting, including outputs from all AI feedback and extracts meaningful insights, tactical analysis, and performance metrics from your replays.

## Features

- Parse Rocket League replay files and extract detailed game statistics and events
- Generate AI-powered analysis using a choice (or all) of ChatGPT, Claude, Gemini or Copilot APIs
- Provide tactical insights and improvement suggestions

## Prerequisites

- Rust (latest stable version)
- Claude, ChatGPT, Gemini and/or Copilot API keys
- Rocket League replay files

## Installation

1. Clone the repository:
```bash
git clone https://github.com/scottleedavis/rocket-league-ai-analysis.git
cd rocket-league-ai-analysis
```

2. Create a `.env` file in the project root.  Any/all key(s) optional for AI feedback:
```bash
CLAUDE_API_KEY="your-api-key-here"
OPENAI_API_KEY="your-api-key-here"
GEMINI_API_KEY="your-api-key-here"
COPILOT_API_KEY="your-api-key-here"
```

3. Build the project:

Locally
```bash
cargo build --release
```

## Usage

```bash
cargo run -- extract path/to/some.replay
cargo run -- convert output/some.replay.json
cargo run -- analyze output/some.replay.json.csv
```


## Testing

Extract Replay Data:
```bash
cargo test
```

Ensure .env is set up properly.
Test with one or both API keys to ensure fallback mechanisms work.

## Configuration

The analyzer can be configured through command-line arguments or environment variables:

Claude
- `CLAUDE_API_KEY`: Your Claude API key
- `RL_ANALYZER_MODEL`: Claude model to use (default: "claude-3-sonnet-20240229")
- `RL_ANALYZER_LOG_LEVEL`: Log level (default: "info")

ChatGPT
-`OPENAI_API_KEY`: Your ChatGPT API key 
- ..

Gemini
- ..

Copilot
- ..



## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [rattletrap](https://github.com/tfausak/rattletrap) - Rocket League replay parser
- [rocketleague-replay-coach](https://github.com/scottleedavis/rocketleague-replay-coach) - ChatGPT proof of concept with python
- [Anthropic](https://anthropic.com) - Claude AI API
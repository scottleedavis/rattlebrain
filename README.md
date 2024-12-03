# rl_replay_ai

A command-line Rust-based replay analysis tool that analyzes Rocket League replay files using one or more AI services.  It utilizes smart environment variable detection for API keys to dynamically select AI interfaces (e.g., OpenAI, Claude).
It provides unified reporting, including outputs from all AI feedback and extracts meaningful insights, tactical analysis, and performance metrics from your replays.

## Features

- Parse Rocket League replay files and extract detailed game statistics and events
- Generate AI-powered analysis using a choice (or all) of ChatGPT, Claude, Gemini or Copilot APIs
- Provide tactical insights and improvement suggestions

## cli Usage

```
./rl_replay_ai analysis /path/or/url/to/some.replay
```

## Local Build Prerequisites

- Rust (latest stable version)
- Claude, ChatGPT, Gemini and/or Copilot API keys
- Rocket League replay files

## Local Installation

1. Clone the repository:
```bash
git clone https://github.com/scottleedavis/rl_replay_ai.git
cd rl_replay_ai
```

2. Create a `.env` file in the project root.  Any/all key(s) optional for AI feedback:
```bash
CLAUDE_API_KEY="your-api-key-here"
OPENAI_API_KEY="your-api-key-here"
GEMINI_API_KEY="your-api-key-here"
COPILOT_API_KEY="your-api-key-here"
```

3. Build with cargo:
```bash
cargo build --release
```

Running with cargo
```bash
cargo run -- analysis path/to/some.replay
# or
cargo run -- analysis http://url/to/some.replay
```

Testing:
```bash
cargo test
```

Ensure .env is set up properly.
Test with one or both API keys to ensure fallback mechanisms work.


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
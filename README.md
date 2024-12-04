# RattleBrain

**RattleBrain** is a command-line tool, written in Rust, designed to bring the power of AI to your **Rocket League** replay analysis. By leveraging **[Rattletrap](https://github.com/tfausak/rattletrap)** for parsing replay files and connecting to one or more AI services (e.g., OpenAI, Claude), RattleBrain extracts tactical insights, performance metrics, and actionable feedback to help you elevate your gameplay.

see the current [examples/feedback.md](examples/feedback.md) for an example of generated output.

also see the current [TODOs](TODO.md) for upcoming features.

## Usage
[Download](https://github.com/scottleedavis/rattlebrain/releases) and Run the latest release of **RattleBrain** with a replay file
```bash
./rattlebrain analyze <replay_file>
```

---

## Features

- **AI-Enhanced Analysis**: Analyze Rocket League replay files with the help of AI services for in-depth insights.
- **Dynamic AI Selection**: Automatically detects and configures available AI services via environment variables (e.g., `OPENAI_API_KEY`, `CLAUDE_API_KEY`).
- **Unified Reporting**: Combines outputs from multiple AI agents into a single, comprehensive report.
- **Tactical Insights**: Highlights key plays, strategic opportunities, and areas for improvement.
- **Performance Metrics**: Extracts useful data points to track your progress over time.
- **Lightweight and Fast**: Built with Rust for performance and efficiency.

---

## Getting Started

### Prerequisites
1. Install **Rattletrap**:
   - RattleBrain uses [Rattletrap](https://github.com/tfausak/rattletrap/releases) to parse replay files, and is automatically downloaded if not detected in $PATH.  

2. Obtain API Keys:
   - Get API keys for the AI services you'd like to use (e.g., OpenAI, Claude).

3. Set Environment Variables:
   - Add your API keys to your environment variables:
     ```bash
     export OPENAI_API_KEY=<your_openai_api_key>
     export CLAUDE_API_KEY=<your_claude_api_key>
     ```

---

## How It Works

1. **Parsing Replays**: 
   RattleBrain uses **Rattletrap** to decode Rocket League replay files into a structured format.

2. **AI Integration**: 
   - Depending on the environment variables detected, RattleBrain connects to one or more AI services.
   - Each AI service processes the replay data, generating unique feedback and insights.

3. **Unified Report**:
   - Outputs from all AI services are combined into a single, readable report, highlighting tactical analysis, performance metrics, and key observations.

---

### Installation
Clone the repository and build **RattleBrain**:
```bash
git clone https://github.com/scottleedavis/rattlebrain.git
cd rattlebrain
```
#### Building

```bash
cargo build --release
```

#### Testing

```bash
cargo test
```

---

## Acknowledgments

- **[Rattletrap](https://github.com/tfausak/rattletrap)**: RattleBrain wouldn’t be possible without this fantastic replay parser. Kudos to the creators and maintainers for providing such a robust tool!
- **AI Service Providers**: OpenAI, Claude, and others for their advanced language models powering this tool.

---

## Contributing

Contributions are welcome! Please open an issue or submit a pull request if you’d like to improve RattleBrain.

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

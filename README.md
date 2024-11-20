# Rocket League Replay Analyzer

## Overview
This project provides a comprehensive tool for analyzing Rocket League replays, leveraging:
- Carball for replay parsing
- Claude AI for in-depth game analysis
- Python for data processing

## Features
- Parse Rocket League replay files
- Extract detailed game statistics
- Generate AI-powered performance insights
- Comprehensive player and team analysis

## Prerequisites
- Python 3.8+
- Anthropic API Key
- Rocket League Replay File

## Installation
1. Clone the repository
```bash
git clone https://github.com/yourusername/rocket-league-replay-analyzer.git
cd rocket-league-replay-analyzer
```

2. Create a virtual environment
```bash
python -m venv venv
source venv/bin/activate  # On Windows use `venv\Scripts\activate`
```

3. Install dependencies
```bash
pip install -r requirements.txt
```

4. Set up environment variables
```bash
cp .env.example .env
# Edit .env and add your Anthropic API Key
```

## Usage
```bash
python -m rocket_league_analyzer.main --replay path/to/replay.replay
```

## Development
- Run tests: `pytest tests/`
- Lint code: `flake8 rocket_league_analyzer`

## Contributing
1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

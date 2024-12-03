# Understand the Replay Data
Examine the CSV to understand its structure. Look for:
1) Player actions (boost usage, positioning, goals, etc.)
* find boost
* find goals

2) Game events:
* find kickoffs
* find shots
* find saves

3) Timestamps or game frames
4) Positional data (x, y, z coordinates)

# Define Coaching Goals

Decide what kind of feedback you're looking for, such as:

Strategy: Positional analysis, rotations, or team synergy.
Mechanics: Boost efficiency, aerial control, or shot accuracy.
Decision-Making: Situational awareness or risk/reward trade-offs.

# Summarize Key Data
A CSV file with raw replay data can be too detailed for a language model to interpret directly. Use Python or a similar tool to preprocess the data and extract key insights:

Aggregate statistics (e.g., average speed, boost usage).
Key events (e.g., missed shots, demo plays, goals).
Player-specific data (e.g., rotations, ball touches).

# Generate Queries for the API
Use the preprocessed data to formulate questions or summaries. For example:

"Player A missed 5 aerials during the match. How can they improve their aerial control?"
"Player B used boost excessively during rotations. What are some tips for efficient boost management?"
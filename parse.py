import json
from math import ceil

# Load the replay JSON
with open('replay.json') as file:
    replay_data = json.load(file)

# Define chunking function
def chunk_frames(frames, chunk_size=100):
    total_frames = len(frames)
    chunks = [
        frames[i:i + chunk_size] for i in range(0, total_frames, chunk_size)
    ]
    return chunks

# Extract key data
metadata = replay_data.get('properties', {})
goals = metadata.get('Goals', [])
player_stats = metadata.get('PlayerStats', [])
frames = replay_data.get('network_frames', {}).get('frames', [])

# Chunk frames for API processing
frame_chunks = chunk_frames(frames, chunk_size=500)

# Format for AI API
formatted_data = {
    "metadata": metadata,
    "goals": goals,
    "player_stats": player_stats,
    "frame_chunks": frame_chunks
}

with open(f'metadata.json','w') as meta:
    json.dump(metadata, meta, indent=4)
with open(f'goals.json','w') as meta:
    json.dump(goals, meta, indent=4)
with open(f'player_stats.json','w') as meta:
    json.dump(player_stats, meta, indent=4)

# Save as individual API-friendly files
for i, chunk in enumerate(frame_chunks):
    with open(f'frame_chunk_{i}.json', 'w') as chunk_file:
        json.dump(chunk, chunk_file, separators=(',', ':'))

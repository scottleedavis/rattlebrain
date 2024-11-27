use std::fs::File;
use std::io::Read;

pub fn extract_replay(input: &str, output: &str) {
    let mut file = File::open(input).expect("Failed to open replay file");
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).expect("Failed to read file");

    // Placeholder: Extract data from replay file (parsing logic TBD)
    println!("Extracting replay data...");

    // Save to CSV
    std::fs::write(output, "Player,Score,Goals\n").expect("Failed to write CSV file");
    println!("Replay data saved to {}", output);
}

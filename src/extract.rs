use rattletrap::Replay;
use std::fs::File;
use std::io::{self, Read, Write};

/// Parses a Rocket League replay file and saves the extracted data to a CSV file.
pub fn extract_replay(input: &str, output: &str) -> io::Result<()> {
    // Open the input replay file
    let mut file = File::open(input)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Parse the replay using rattletrap
    let replay = rattletrap::de::from_bytes(&buffer)?;

    // Write the parsed data to a CSV file (basic stub implementation)
    let mut output_file = File::create(output)?;
    writeln!(output_file, "Property,Value")?;
    writeln!(output_file, "Replay Version,{}", replay.header.version)?;

    println!("Replay data successfully written to {}", output);
    Ok(())
}

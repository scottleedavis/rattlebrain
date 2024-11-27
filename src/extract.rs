use std::process::Command;
use std::fs::{self, File};
use std::io::{self, Write};

/// Parses a Rocket League replay file using the `rattletrap` CLI and writes the result to a CSV file.
pub fn extract_replay(input: &str, output: &str) -> io::Result<()> {
    let rattletrap_path = "./rattletrap"; // Adjust this path if necessary

    // Temporary JSON output from rattletrap
    let temp_output = format!("{}.json", output);

    // Run the rattletrap command
    let output_status = Command::new(rattletrap_path)
        .arg("--input")
        .arg(input)
        .arg("--output")
        .arg(&temp_output)
        .output();

    match output_status {
        Ok(output) => {
            if !output.status.success() {
                eprintln!(
                    "Failed to extract replay data. Error: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
                return Err(io::Error::new(io::ErrorKind::Other, "Rattletrap failed"));
            }
        }
        Err(e) => {
            eprintln!("Failed to execute rattletrap: {}", e);
            return Err(e);
        }
    }

    // Read the JSON file and convert it to CSV format
    let json_data = fs::read_to_string(&temp_output)?;
    let parsed_data: serde_json::Value = serde_json::from_str(&json_data)?;

    let mut csv_file = File::create(output)?;
    writeln!(csv_file, "Property,Value")?;
    for (key, value) in parsed_data.as_object().unwrap() {
        writeln!(csv_file, "{},{}", key, value)?;
    }

    // Clean up temporary JSON file
    fs::remove_file(temp_output)?;

    println!("Replay data successfully written to {}", output);
    Ok(())
}

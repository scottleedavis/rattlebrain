use std::fs::File;
use std::io::{self, Read, Write};

pub fn extract_replay(input: &str, output: &str) -> io::Result<()> {
    // Open and read the replay file
    let mut file = File::open(input)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    // Placeholder: Simulate extracted data
    let data = vec![
        vec!["Player".to_string(), "Goals".to_string(), "Assists".to_string()],
        vec!["Player1".to_string(), "2".to_string(), "1".to_string()],
        vec!["Player2".to_string(), "1".to_string(), "2".to_string()],
    ];

    // Write the data to a CSV file
    let mut output_file = File::create(output)?;
    for row in data {
        writeln!(output_file, "{}", row.join(","))?;
    }

    println!("Replay data successfully extracted to {}", output);
    Ok(())
}

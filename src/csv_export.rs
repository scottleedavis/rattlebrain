use std::fs::File;
use std::io::Write;

pub fn export_to_csv(data: Vec<Vec<String>>, output: &str) {
    let mut file = File::create(output).expect("Failed to create CSV file");
    for row in data {
        writeln!(file, "{}", row.join(",")).expect("Failed to write CSV file");
    }
    println!("Data exported to CSV at {}", output);
}

use boxcars::ParserBuilder;
use std::fs::File;
use std::io::{self, Read};

fn main() {
    // Specify the file path (change this to the desired file path)
    let file_path = "/mnt/c/Users/scott/Documents/workspace/rocket-league-replay-analysis-claude/test.replay";

    // Open the file and read its contents into a buffer
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("Failed to read file");

    // Parse the replay data
    let replay = ParserBuilder::new(&data)
        .always_check_crc()
        .must_parse_network_data()
        .parse()
        .expect("Failed to parse replay");

    // Write the parsed replay data as pretty JSON to stdout
    let stdout = io::stdout();
    let mut out = stdout.lock();
    serde_json::to_writer_pretty(&mut out, &replay).expect("Failed to write JSON");
}

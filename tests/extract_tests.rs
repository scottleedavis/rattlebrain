#[cfg(test)]
mod tests {
    use super::super::replay_parser;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_extract_valid_replay() {
        // Create a temporary valid replay file
        let valid_replay_path = "tests/valid.replay";
        let output_path = "tests/output.csv";
        let mut file = File::create(valid_replay_path).unwrap();
        file.write_all(&[/* Insert valid replay bytes here */]).unwrap();

        let result = replay_parser::extract_replay(valid_replay_path, output_path);
        assert!(result.is_ok());

        // Verify output file
        let output_contents = std::fs::read_to_string(output_path).unwrap();
        assert!(output_contents.contains("Replay Version"));

        // Clean up
        std::fs::remove_file(valid_replay_path).unwrap();
        std::fs::remove_file(output_path).unwrap();
    }

    #[test]
    fn test_extract_invalid_replay() {
        // Create a temporary invalid replay file
        let invalid_replay_path = "tests/invalid.replay";
        let mut file = File::create(invalid_replay_path).unwrap();
        file.write_all(b"Invalid replay data").unwrap();

        let result = replay_parser::extract_replay(invalid_replay_path, "tests/output.csv");
        assert!(result.is_err());

        // Clean up
        std::fs::remove_file(invalid_replay_path).unwrap();
    }
}

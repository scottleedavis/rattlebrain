#[cfg(test)]
mod analysis_tests {
    use std::process::Command;

    #[test]
    fn test_analysis_command() {
        let output = Command::new("cargo")
            .args(&["run", "--", "analysis", "examples/sample.replay"])
            .output()
            .expect("Failed to execute process");

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        println!("STDOUT: {}", stdout);
        println!("STDERR: {}", stderr);

        // Check for expected output
        assert!(stdout.contains("Analyzing replay file: examples/sample.replay"));
        assert!(stdout.contains("Analysis completed! This is just a stub for now."));
    }
}

#[test]
fn test_extract_command() {
    let output = std::process::Command::new("cargo")
        .args(&["run", "--", "extract", "input.replay", "output.csv"])
        .output()
        .expect("Failed to execute process");

    // Print captured stdout and stderr for debugging
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("STDOUT: {}", stdout);
    println!("STDERR: {}", stderr);

    // Assert that stdout contains the expected output
    assert!(stdout.contains("Extracting replay data..."));
}

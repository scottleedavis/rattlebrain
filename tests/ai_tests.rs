#[cfg(test)]
mod ai_tests {
    use std::process::Command;
    use std::env;


    #[test]
    fn test_ai_command_with_openai() {
        std::env::set_var("OPENAI_API_KEY", "test-openai-key");
        
        let output = std::process::Command::new("cargo")
            .args(&["run", "--", "ai", "test_query"])
            .output()
            .expect("Failed to execute process");

        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("STDOUT: {}", stdout);

        assert!(stdout.contains("OpenAI response to 'test_query': [This is a stubbed response from OpenAI]"));

        std::env::remove_var("OPENAI_API_KEY");
    }


    #[test]
    fn test_ai_command_with_claude() {
        env::set_var("CLAUDE_API_KEY", "test-claude-key");
        let output = Command::new("cargo")
            .args(&["run", "--", "ai", "test_query"])
            .output()
            .expect("Failed to execute process");

        let stdout = String::from_utf8_lossy(&output.stdout);

        assert!(stdout.contains("Processing AI query: test_query"));
        assert!(stdout.contains("Claude response to 'test_query': [This is a stubbed response from Claude]"));

        env::remove_var("CLAUDE_API_KEY");
    }

    #[test]
    fn test_ai_command_with_gemini() {
        env::set_var("GEMINI_API_KEY", "test-gemini-key");
        let output = Command::new("cargo")
            .args(&["run", "--", "ai", "test_query"])
            .output()
            .expect("Failed to execute process");

        let stdout = String::from_utf8_lossy(&output.stdout);

        assert!(stdout.contains("Processing AI query: test_query"));
        assert!(stdout.contains("Gemini response to 'test_query': [This is a stubbed response from Gemini]"));

        env::remove_var("GEMINI_API_KEY");
    }

    #[test]
    fn test_ai_command_with_copilot() {
        env::set_var("COPILOT_API_KEY", "test-copilot-key");
        let output = Command::new("cargo")
            .args(&["run", "--", "ai", "test_query"])
            .output()
            .expect("Failed to execute process");

        let stdout = String::from_utf8_lossy(&output.stdout);

        assert!(stdout.contains("Processing AI query: test_query"));
        assert!(stdout.contains("GitHub Copilot response to 'test_query': [This is a stubbed response from Copilot]"));

        env::remove_var("COPILOT_API_KEY");
    }

    // #[test]
    // fn test_ai_command_with_no_providers() {
    //     // Unset any existing AI provider environment variables
    //     std::env::remove_var("OPENAI_API_KEY");
    //     std::env::remove_var("CLAUDE_API_KEY");
    //     std::env::remove_var("GEMINI_API_KEY");
    //     std::env::remove_var("COPILOT_API_KEY");

    //     let output = std::process::Command::new("cargo")
    //         .args(&["run", "--", "ai", "test_query"])
    //         .output()
    //         .expect("Failed to execute process");

    //     let stdout = String::from_utf8_lossy(&output.stdout);
    //     let stderr = String::from_utf8_lossy(&output.stderr);

    //     println!("STDOUT: {}", stdout);
    //     println!("STDERR: {}", stderr);

    //     // Check for expected error message
    //     assert!(stderr.contains("No AI providers configured."));
    // }

}

use std::env;
use std::io;

pub fn query_ai(query: &str) -> io::Result<String> {
    println!("Processing AI query: {}", query);

    // Detect available AI providers and collect responses
    let mut responses = Vec::new();

    // OpenAI
    if let Ok(openai_key) = env::var("OPENAI_API_KEY") {
        println!("Using OpenAI with key: {}", &openai_key[0..4]);
        responses.push(format!(
            "OpenAI response to '{}': [This is a stubbed response from OpenAI]",
            query
        ));
    }

    // Claude
    if let Ok(claude_key) = env::var("CLAUDE_API_KEY") {
        println!("Using Claude with key: {}", &claude_key[0..4]);
        responses.push(format!(
            "Claude response to '{}': [This is a stubbed response from Claude]",
            query
        ));
    }

    // Gemini
    if let Ok(gemini_key) = env::var("GEMINI_API_KEY") {
        println!("Using Gemini with key: {}", &gemini_key[0..4]);
        responses.push(format!(
            "Gemini response to '{}': [This is a stubbed response from Gemini]",
            query
        ));
    }

    // GitHub Copilot
    if let Ok(copilot_key) = env::var("COPILOT_API_KEY") {
        println!("Using GitHub Copilot with key: {}", &copilot_key[0..4]);
        responses.push(format!(
            "GitHub Copilot response to '{}': [This is a stubbed response from Copilot]",
            query
        ));
    }

    // Handle case where no providers are configured
    if responses.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "No AI providers configured. Please set one or more of the following: OPENAI_API_KEY, CLAUDE_API_KEY, GEMINI_API_KEY, COPILOT_API_KEY.",
        ));
    }


    // Combine all responses
    let combined_response = responses.join("\n");
    Ok(combined_response)
}

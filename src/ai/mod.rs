pub mod openai;
pub mod claude;
pub mod generic;

pub fn generate_ai_feedback(input: &str) {
    use crate::ai::generic::AIInterface;

    println!("Generating AI feedback...");

    if let Ok(openai_key) = std::env::var("OPENAI_API_KEY") {
        let feedback = openai::OpenAI::new(openai_key).query(input);
        println!("OpenAI Feedback: {}", feedback.unwrap_or("Failed".to_string()));
    }

    if let Ok(claude_key) = std::env::var("CLAUDE_API_KEY") {
        let feedback = claude::Claude::new(claude_key).query(input);
        println!("Claude Feedback: {}", feedback.unwrap_or("Failed".to_string()));
    }
}

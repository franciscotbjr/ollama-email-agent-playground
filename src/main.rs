mod config;
use ollama_ai_agents_playground::agent::{Agent, classifier::IntentClassifierAgent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a tokio runtime for the async example
    println!();
    println!("🚀 Starting asynchronous processing...");
    println!("🚀 Starting classifier...");
    println!();
    let input = "Envie um e-mail para Turtle informando que não vou poder comparecer à reunião e que peço desculpas por avisar tão em cima da hora.";
    let intent_classifier_agent = IntentClassifierAgent::new();
    let result = intent_classifier_agent.process(input).await;
    match result {
        Ok(classification_result) => {
            println!();
            println!("🚀 Classification done!");
            println!("User intent: {}", classification_result.intent);
            println!("User recipient: {}", classification_result.params.recipient().unwrap());
            println!();
        }
        Err(e) => {
            println!("Failed: {}", e);
        }
    }

    Ok(())
}

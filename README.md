# Ollama AI Agents Playground

A Rust-based AI agent system that leverages Ollama for intent classification and task automation. This project demonstrates a modular architecture for building AI-powered agents that can classify user intents and perform automated actions.

## Features

- **Intent Classification**: Uses Ollama with Gemma3 model to classify user intents
- **Modular Agent Architecture**: Extensible system for creating specialized AI agents
- **HTTP Client Infrastructure**: Robust HTTP communication layer
- **Configuration Management**: Lazy-loaded configuration system
- **Comprehensive Testing**: Unit tests for all major components

## Supported Intents

- **SendEmail**: Classifies requests to send emails
- **ScheduleMeeting**: Identifies meeting scheduling requests
- **NoAction**: Fallback for unrecognized intents

## Prerequisites

### Ollama Installation

You must have Ollama installed and running locally on your system.

1. **Install Ollama**: Visit [Ollama's official website](https://ollama.ai/) and follow the installation instructions for your platform.

2. **Pull the Gemma3 model**:
   ```bash
   ollama pull gemma3
   ```

3. **Verify Ollama is running**:
   ```bash
   ollama list
   ```
   You should see `gemma3` in the model list.

### Rust Environment

- Rust 1.70+ (2024 edition)
- Cargo package manager

## Quick Start

1. **Clone the repository**:
   ```bash
   git clone <repository-url>
   cd ollama-ai-agents-playground
   ```

2. **Configure the application**:
   Edit `config.toml` to match your environment:
   ```toml
   [database]
   path = "path/to/your/database.db"

   [ollama.api]
   url = "http://localhost:11434/api/chat"
   model = "gemma3"
   ```

3. **Build and run**:
   ```bash
   cargo build
   cargo run
   ```

## Project Structure

```
src/
├── agent/                    # Agent system
│   ├── classifier/          # Intent classification agent
│   │   ├── classifier_agent.rs    # Main classifier implementation
│   │   ├── classification_result.rs # Classification result types
│   │   ├── params.rs              # Parameter extraction
│   │   └── response_mapper.rs     # Response mapping utilities
│   ├── email/               # Email agent (future implementation)
│   └── intent.rs            # Intent enumeration
├── infra/                   # Infrastructure layer
│   ├── http/                # HTTP client infrastructure
│   ├── ollama/              # Ollama API integration
│   └── contacts/            # Contact management (future)
├── config.rs                # Configuration management
└── main.rs                  # Demo application
```

## Usage Example

The demo application showcases the ClassifierAgent in action:

```rust
use ollama_ai_agents_playground::agent::{Agent, classifier::ClassifierAgent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = "Send an email to Turtle informing that I won't be able to attend the meeting";
    
    let classifier_agent = ClassifierAgent::new();
    let result = classifier_agent.process(input).await;
    
    match result {
        Ok(classification_result) => {
            println!("User intent: {}", classification_result.intent);
        }
        Err(e) => {
            println!("Classification failed: {}", e);
        }
    }
    
    Ok(())
}
```

## Configuration

The application uses a TOML configuration file (`config.toml`) with lazy loading for optimal performance:

- **Database**: SQLite database path for future persistence features
- **Ollama API**: URL and model configuration for AI processing

## Testing

Run the comprehensive test suite:

```bash
# Run all tests
cargo test

# Run specific test modules
cargo test config::
cargo test ollama_request::
cargo test ollama_message::
```

The project includes 100+ unit tests covering:
- Configuration loading and validation
- HTTP client functionality
- Ollama request/response handling
- Intent classification components
- Serialization/deserialization

## Development

### Adding New Agents

1. Create a new module in `src/agent/`
2. Implement the `Agent` trait:
   ```rust
   impl Agent for YourAgent {
       fn process(&self, input: &str) -> impl Future<Output = Result<ClassificationResult, AgentError>> + Send {
           // Your implementation
       }
   }
   ```

### Extending Intent Types

Add new variants to the `Intent` enum in `src/agent/intent.rs`:

```rust
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Intent {
    SendEmail,
    ScheduleMeeting,
    YourNewIntent,  // Add here
    NoAction,
}
```

## Useful Links

- [Ollama Official Website](https://ollama.ai/)
- [Ollama GitHub Repository](https://github.com/ollama/ollama)
- [Gemma3 Model Documentation](https://ollama.ai/library/gemma3)
- [Ollama API Documentation](https://github.com/ollama/ollama/blob/main/docs/api.md)

## Dependencies

- **serde**: Serialization framework
- **tokio**: Async runtime
- **reqwest**: HTTP client
- **once_cell**: Lazy static initialization
- **toml**: Configuration file parsing

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass: `cargo test`
5. Submit a pull request

## Troubleshooting

### Common Issues

1. **Ollama not responding**: Ensure Ollama is running (`ollama serve`)
2. **Model not found**: Pull the gemma3 model (`ollama pull gemma3`)
3. **Connection refused**: Check if the URL in `config.toml` matches your Ollama installation
4. **Build errors**: Ensure you're using Rust 2024 edition (1.70+)

### Debug Mode

Run with detailed logging:
```bash
RUST_LOG=debug cargo run
```
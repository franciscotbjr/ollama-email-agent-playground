use crate::agent::Intent;
use crate::agent::classifier::Params;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OllamaResponseContent {
    pub intent: Intent,
    pub params: Params,
}

impl OllamaResponseContent {
    /// Extracts JSON from ```json ... ``` markdown format and parses it
    pub fn from_markdown_json(content: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json_content = Self::extract_json_from_markdown(content)?;
        let parsed: OllamaResponseContent = serde_json::from_str(&json_content)?;
        Ok(parsed)
    }

    /// Extracts JSON content from markdown code block
    fn extract_json_from_markdown(content: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Find the start and end of the JSON code block
        if let Some(start) = content.find("```json") {
            let after_start = &content[start + 7..]; // Skip "```json"
            if let Some(end) = after_start.find("```") {
                let json_content = &after_start[..end].trim();
                return Ok(json_content.to_string());
            }
        }

        // Fallback: try to find JSON without markdown markers
        if content.trim().starts_with('{') && content.trim().ends_with('}') {
            return Ok(content.trim().to_string());
        }

        Err(format!("Could not extract JSON from content: {}", content).into())
    }

    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_json_from_markdown() {
        let markdown_content = r#"```json
{
  "intent": "SendEmail",
  "params": {
    "recipient": "Turtle",
    "message": "informing her that I won't be able to attend the meeting"
  }
}
```"#;

        let result = OllamaResponseContent::extract_json_from_markdown(markdown_content).unwrap();
        assert!(result.contains("SendEmail"));
        assert!(result.contains("Turtle"));
    }

    #[test]
    fn test_from_markdown_json() {
        let markdown_content = r#"```json
{
  "intent": "SendEmail",
  "params": {
    "recipient": "Turtle",
    "message": "informing her that I won't be able to attend the meeting"
  }
}
```"#;

        let result = OllamaResponseContent::from_markdown_json(markdown_content).unwrap();
        assert_eq!(result.intent, Intent::SendEmail);
        assert_eq!(result.params.recipient(), Some("Turtle"));
        assert_eq!(
            result.params.message(),
            Some("informing her that I won't be able to attend the meeting")
        );
    }

    #[test]
    fn test_fallback_plain_json() {
        let plain_json = r#"{
  "intent": "ScheduleMeeting",
  "params": {
    "recipient": "John",
    "message": "Let's schedule a meeting"
  }
}"#;

        let result = OllamaResponseContent::from_markdown_json(plain_json).unwrap();
        assert_eq!(result.intent, Intent::ScheduleMeeting);
    }

    #[test]
    fn test_invalid_content() {
        let invalid_content = "This is not JSON content";
        let result = OllamaResponseContent::from_markdown_json(invalid_content);
        assert!(result.is_err());
    }
}

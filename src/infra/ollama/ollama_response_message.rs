use super::ollama_response_content::OllamaResponseContent;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OllamaResponseMessage {
    pub role: String,
    #[serde(rename = "content")]
    raw_content: String,
}

impl OllamaResponseMessage {
    /// Returns the raw content string as received from Ollama
    pub fn raw_content(&self) -> &str {
        &self.raw_content
    }

    /// Parses the content to extract structured JSON data
    pub fn parsed_content(&self) -> Result<OllamaResponseContent, Box<dyn std::error::Error>> {
        OllamaResponseContent::from_markdown_json(&self.raw_content)
    }

    /// Convenience method to get content, trying parsed first, fallback to raw
    pub fn content(&self) -> String {
        match self.parsed_content() {
            Ok(parsed) => parsed
                .to_json_string()
                .unwrap_or_else(|_| self.raw_content.clone()),
            Err(_) => self.raw_content.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::Intent;

    fn create_test_message(content: &str) -> OllamaResponseMessage {
        OllamaResponseMessage {
            role: "assistant".to_string(),
            raw_content: content.to_string(),
        }
    }

    #[test]
    fn test_raw_content() {
        let raw_text = "This is raw content from Ollama";
        let message = create_test_message(raw_text);

        assert_eq!(message.raw_content(), raw_text);
    }

    #[test]
    fn test_raw_content_with_markdown_json() {
        let markdown_content = r#"```json
{
  "intent": "SendEmail",
  "params": {
    "recipient": "test@example.com",
    "message": "Hello world"
  }
}
```"#;
        let message = create_test_message(markdown_content);

        assert_eq!(message.raw_content(), markdown_content);
    }

    #[test]
    fn test_parsed_content_success() {
        let markdown_content = r#"```json
{
  "intent": "SendEmail",
  "params": {
    "recipient": "turtle.patient@wildkingdom.org",
    "message": "Meeting update"
  }
}
```"#;
        let message = create_test_message(markdown_content);

        let parsed = message.parsed_content().unwrap();
        assert_eq!(parsed.intent, Intent::SendEmail);
        assert_eq!(parsed.params.recipient(), Some("turtle.patient@wildkingdom.org"));
        assert_eq!(parsed.params.message(), Some("Meeting update"));
    }

    #[test]
    fn test_parsed_content_with_plain_json() {
        let plain_json = r#"{
  "intent": "ScheduleMeeting",
  "params": {
    "recipient": "john@example.com",
    "message": "Let's schedule a meeting"
  }
}"#;
        let message = create_test_message(plain_json);

        let parsed = message.parsed_content().unwrap();
        assert_eq!(parsed.intent, Intent::ScheduleMeeting);
        assert_eq!(parsed.params.recipient(), Some("john@example.com"));
    }

    #[test]
    fn test_parsed_content_failure() {
        let invalid_content = "This is not valid JSON content";
        let message = create_test_message(invalid_content);

        let result = message.parsed_content();
        assert!(result.is_err());
    }

    #[test]
    fn test_parsed_content_with_no_action() {
        let no_action_json = r#"```json
{
  "intent": "NoAction",
  "params": {
    "recipient": null,
    "message": null
  }
}
```"#;
        let message = create_test_message(no_action_json);

        let parsed = message.parsed_content().unwrap();
        assert_eq!(parsed.intent, Intent::NoAction);
        assert_eq!(parsed.params.recipient(), None);
        assert_eq!(parsed.params.message(), None);
    }

    #[test]
    fn test_content_method_with_valid_json() {
        let markdown_content = r#"```json
{
  "intent": "SendEmail",
  "params": {
    "recipient": "test@example.com",
    "message": "Test message"
  }
}
```"#;
        let message = create_test_message(markdown_content);

        let content = message.content();
        // Should return the serialized JSON, not the original markdown
        assert!(content.contains("SendEmail"));
        assert!(content.contains("test@example.com"));
        assert!(!content.contains("```json")); // Should not contain markdown markers
    }

    #[test]
    fn test_content_method_with_invalid_content() {
        let invalid_content = "This is just plain text, not JSON";
        let message = create_test_message(invalid_content);

        let content = message.content();
        // Should fallback to raw content
        assert_eq!(content, invalid_content);
    }

    #[test]
    fn test_deserialization_from_json() {
        let json_str = r#"{
  "role": "assistant",
  "content": "```json\n{\n  \"intent\": \"SendEmail\",\n  \"params\": {\n    \"recipient\": \"Turtle\",\n    \"message\": \"Meeting canceled\"\n  }\n}\n```"
}"#;

        let message: OllamaResponseMessage = serde_json::from_str(json_str).unwrap();
        assert_eq!(message.role, "assistant");
        assert!(message.raw_content().contains("SendEmail"));
        assert!(message.raw_content().contains("Turtle"));
    }

    #[test]
    fn test_content_with_unicode_characters() {
        let unicode_content = r#"```json
{
  "intent": "SendEmail",
  "params": {
    "recipient": "用户@example.com",
    "message": "Hello 世界! 🌍"
  }
}
```"#;
        let message = create_test_message(unicode_content);

        let parsed = message.parsed_content().unwrap();
        assert_eq!(parsed.params.recipient(), Some("用户@example.com"));
        assert_eq!(parsed.params.message(), Some("Hello 世界! 🌍"));
    }

    #[test]
    fn test_empty_content() {
        let message = create_test_message("");

        assert_eq!(message.raw_content(), "");
        assert!(message.parsed_content().is_err());
        assert_eq!(message.content(), "");
    }

    #[test]
    fn test_malformed_json_in_markdown() {
        let malformed_content = r#"```json
{
  "intent": "SendEmail",
  "params": {
    "recipient": "test@example.com"
    // Missing comma and closing brace
```"#;
        let message = create_test_message(malformed_content);

        assert!(message.parsed_content().is_err());
        assert_eq!(message.content(), malformed_content); // Should fallback to raw
    }
}

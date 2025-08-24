use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Params {
    recipient: Option<String>,
    message: Option<String>,
}

impl Params {
    pub fn new(recipient: Option<String>, message: Option<String>) -> Self {
        Self { recipient, message }
    }

    pub fn with_values(recipient: String, message: String) -> Self {
        Self {
            recipient: Some(recipient),
            message: Some(message),
        }
    }

    pub fn from_json_str(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }

    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn recipient(&self) -> Option<&str> {
        self.recipient.as_deref()
    }

    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_params() {
        let recipient = "test@example.com".to_string();
        let message = "Test message content".to_string();
        let params = Params::new(Some(recipient.clone()), Some(message.clone()));

        assert_eq!(params.recipient(), Some(recipient.as_str()));
        assert_eq!(params.message(), Some(message.as_str()));
    }

    #[test]
    fn test_params_with_empty_strings() {
        let params = Params::new(Some(String::new()), Some(String::new()));

        assert_eq!(params.recipient(), Some(""));
        assert_eq!(params.message(), Some(""));
    }

    #[test]
    fn test_params_with_none_values() {
        let params = Params::new(None, None);

        assert_eq!(params.recipient(), None);
        assert_eq!(params.message(), None);
    }

    #[test]
    fn test_params_with_mixed_values() {
        let params = Params::new(Some("test@example.com".to_string()), None);

        assert_eq!(params.recipient(), Some("test@example.com"));
        assert_eq!(params.message(), None);
    }

    #[test]
    fn test_params_with_unicode_content() {
        let recipient = "用户@example.com".to_string();
        let message = "Hello 世界! 🌍".to_string();
        let params = Params::new(Some(recipient.clone()), Some(message.clone()));

        assert_eq!(params.recipient(), Some(recipient.as_str()));
        assert_eq!(params.message(), Some(message.as_str()));
    }

    #[test]
    fn test_with_values_constructor() {
        let recipient = "test@example.com".to_string();
        let message = "Test message".to_string();
        let params = Params::with_values(recipient.clone(), message.clone());

        assert_eq!(params.recipient(), Some(recipient.as_str()));
        assert_eq!(params.message(), Some(message.as_str()));
    }

    #[test]
    fn test_serialization_to_json() {
        let params = Params::with_values(
            "turtle.patient@wildkingdom.org".to_string(),
            "I won't be able to attend the meeting".to_string(),
        );

        let json_string = params.to_json_string().unwrap();
        assert!(json_string.contains("turtle.patient@wildkingdom.org"));
        assert!(json_string.contains("I won't be able to attend"));
        assert!(json_string.contains("recipient"));
        assert!(json_string.contains("message"));
    }

    #[test]
    fn test_serialization_with_none_values() {
        let params = Params::new(None, Some("message only".to_string()));

        let json_string = params.to_json_string().unwrap();
        assert!(json_string.contains("null"));
        assert!(json_string.contains("message only"));
    }

    #[test]
    fn test_deserialization_from_json() {
        let json_str = r#"
        {
            "recipient": "john@example.com",
            "message": "Meeting postponed until tomorrow"
        }"#;

        let params = Params::from_json_str(json_str).unwrap();
        assert_eq!(params.recipient(), Some("john@example.com"));
        assert_eq!(params.message(), Some("Meeting postponed until tomorrow"));
    }

    #[test]
    fn test_deserialization_with_null_values() {
        let json_str = r#"
        {
            "recipient": null,
            "message": "Message without recipient"
        }"#;

        let params = Params::from_json_str(json_str).unwrap();
        assert_eq!(params.recipient(), None);
        assert_eq!(params.message(), Some("Message without recipient"));
    }

    #[test]
    fn test_roundtrip_serialization() {
        let original = Params::with_values(
            "roundtrip@test.com".to_string(),
            "This message should survive serialization roundtrip".to_string(),
        );

        let json_string = original.to_json_string().unwrap();
        let deserialized = Params::from_json_str(&json_string).unwrap();

        assert_eq!(original.recipient(), deserialized.recipient());
        assert_eq!(original.message(), deserialized.message());
    }

    #[test]
    fn test_roundtrip_with_none_values() {
        let original = Params::new(None, Some("only message".to_string()));

        let json_string = original.to_json_string().unwrap();
        let deserialized = Params::from_json_str(&json_string).unwrap();

        assert_eq!(original.recipient(), deserialized.recipient());
        assert_eq!(original.message(), deserialized.message());
    }

    #[test]
    fn test_invalid_json_deserialization() {
        let invalid_json = r#"{"invalid": "structure"}"#;
        let result = Params::from_json_str(invalid_json);
        // With Option fields, missing fields are allowed (set to None)
        // This test should check for truly invalid JSON structure
        assert!(result.is_ok());
        let params = result.unwrap();
        assert_eq!(params.recipient(), None);
        assert_eq!(params.message(), None);
    }

    #[test]
    fn test_truly_invalid_json() {
        let invalid_json = r#"{"invalid": json structure"#; // Missing quotes and closing brace
        let result = Params::from_json_str(invalid_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_incomplete_json_deserialization() {
        let incomplete_json = r#"{"recipient": "test@example.com"}"#;
        let result = Params::from_json_str(incomplete_json);
        // With Option fields, this should now succeed with message as None
        assert!(result.is_ok());
        let params = result.unwrap();
        assert_eq!(params.recipient(), Some("test@example.com"));
        assert_eq!(params.message(), None);
    }

    #[test]
    fn test_clone_functionality() {
        let original = Params::with_values(
            "clone@test.com".to_string(),
            "Clone this message".to_string(),
        );
        let cloned = original.clone();

        assert_eq!(original.recipient(), cloned.recipient());
        assert_eq!(original.message(), cloned.message());

        // Verify they are independent instances
        assert_eq!(
            original.to_json_string().unwrap(),
            cloned.to_json_string().unwrap()
        );
    }

    #[test]
    fn test_special_characters_in_content() {
        let params = Params::with_values(
            "test+tag@example.com".to_string(),
            r#"Message with "quotes", newlines\n, and \t tabs"#.to_string(),
        );

        let json_string = params.to_json_string().unwrap();
        let deserialized = Params::from_json_str(&json_string).unwrap();

        assert_eq!(params.recipient(), deserialized.recipient());
        assert_eq!(params.message(), deserialized.message());
    }

    #[test]
    fn test_long_content() {
        let long_message = "a".repeat(10000);
        let params = Params::with_values("long@example.com".to_string(), long_message.clone());

        assert_eq!(params.message(), Some(long_message.as_str()));

        let json_string = params.to_json_string().unwrap();
        let deserialized = Params::from_json_str(&json_string).unwrap();
        assert_eq!(deserialized.message(), Some(long_message.as_str()));
    }
}

use serde::{Deserialize, Serialize};

use crate::agent::{AgentResult, Intent, classifier::Params};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClassificationResult {
    pub intent: Intent,
    pub params: Params,
}

impl ClassificationResult {
    pub fn new(intent: Intent, params: Params) -> Self {
        Self { intent, params }
    }

    pub fn from_json_str(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }

    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

impl AgentResult for ClassificationResult {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::Intent;

    #[test]
    fn test_new_classification_result() {
        let params =
            Params::with_values("test@example.com".to_string(), "Test message".to_string());
        let result = ClassificationResult::new(Intent::SendEmail, params.clone());

        assert_eq!(result.intent, Intent::SendEmail);
        assert_eq!(
            result.params.to_json_string().unwrap(),
            params.to_json_string().unwrap()
        );
    }

    #[test]
    fn test_serialization_to_json() {
        let params = Params::with_values(
            "turtle.patient@wildkingdom.org".to_string(),
            "informing her that I won't be able to attend the meeting".to_string(),
        );
        let result = ClassificationResult::new(Intent::SendEmail, params);

        let json_string = result.to_json_string().unwrap();
        assert!(json_string.contains("SendEmail"));
        assert!(json_string.contains("turtle.patient@wildkingdom.org"));
        assert!(json_string.contains("informing her"));
    }

    #[test]
    fn test_deserialization_from_json() {
        let json_str = r#"
        {
            "intent": "SendEmail",
            "params": {
                "recipient": "test@example.com",
                "message": "Hello world"
            }
        }"#;

        let result = ClassificationResult::from_json_str(json_str).unwrap();
        assert_eq!(result.intent, Intent::SendEmail);
    }

    #[test]
    fn test_roundtrip_serialization() {
        let original_params = Params::with_values(
            "original@test.com".to_string(),
            "Original message content".to_string(),
        );
        let original = ClassificationResult::new(Intent::ScheduleMeeting, original_params);

        let json_string = original.to_json_string().unwrap();
        let deserialized = ClassificationResult::from_json_str(&json_string).unwrap();

        assert_eq!(original.intent, deserialized.intent);
        assert_eq!(
            original.params.to_json_string().unwrap(),
            deserialized.params.to_json_string().unwrap()
        );
    }

    #[test]
    fn test_all_intent_types() {
        let params = Params::with_values("test@example.com".to_string(), "test".to_string());

        let send_email = ClassificationResult::new(Intent::SendEmail, params.clone());
        assert_eq!(send_email.intent, Intent::SendEmail);

        let schedule_meeting = ClassificationResult::new(Intent::ScheduleMeeting, params.clone());
        assert_eq!(schedule_meeting.intent, Intent::ScheduleMeeting);

        let no_action = ClassificationResult::new(Intent::NoAction, params);
        assert_eq!(no_action.intent, Intent::NoAction);
    }

    #[test]
    fn test_invalid_json_deserialization() {
        let invalid_json = r#"{"invalid": "json"}"#;
        let result = ClassificationResult::from_json_str(invalid_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_clone_functionality() {
        let params = Params::with_values("clone@test.com".to_string(), "Clone test".to_string());
        let original = ClassificationResult::new(Intent::SendEmail, params);
        let cloned = original.clone();

        assert_eq!(original.intent, cloned.intent);
        assert_eq!(
            original.params.to_json_string().unwrap(),
            cloned.params.to_json_string().unwrap()
        );
    }
}

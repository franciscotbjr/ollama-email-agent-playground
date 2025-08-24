# SPECIFICATION FOR AI AGENTS WITH PERSONAL ASSISTANT ROLE

This personal assistant library in Rust operates primarily in a local environment 
with Ollama and the Gemma3 model.

## FIRST STEP - FIRST AGENT

User message classifier.

   - Must identify the user's intent among three possibilities:
      - send_email
      - schedule_meeting 
      - no_action

### USER MESSAGE
```text
Send an email to Turtle informing her that I won't be able to attend the meeting and that I apologize for giving such short notice.
```
```text
Envie um e-mail para Turtle informando que não vou poder comparecer à reunião e que peço desculpas por avisar tão em cima da hora.
```

### REQUEST REFERENCE SAMPLE
```json
{
  "model": "{{model}}", 
  "messages": [
    {
      "role": "user", 
      "content": "Classify intent and extract parameters (JSON format):        Example 1:        Input: \"Send an email to Carlos about the delay\"        Output: {\"intent\":\"send_email\", \"params\":{\"recipient\":\"Carlos\",\"message\":\"About the delay\"}}                Example 2:        Input: \"Send message to Sofia: I'll arrive in 10 min\"        Output: {\"intent\":\"send_message\", \"params\":{\"recipient\":\"Sofia\",\"message\":\"I'll arrive in 10 min\"}}                Task: Return JSON with: action (send_email, schedule_meeting, no_action)       Input: \"Send an email to Turtle informing her that I won't be able to attend the meeting and that I apologize for giving such short notice.\"        Output: " 
    }
  ],
  "stream": false,
  "think": false 
}
```

### RESPONSE REFERENCE SAMPLE
```json
{
    "model": "gemma3",
    "created_at": "2025-08-17T17:48:31.6221749Z",
    "message": {
        "role": "assistant",
        "content": "```json\n{\n  \"intent\": \"send_email\",\n  \"params\": {\n    \"recipient\": \"Turtle\",\n    \"message\": \"informing her that I won't be able to attend the meeting and that I apologize for giving such short notice.\"\n  }\n}\n```\n"
    },
    "done_reason": "stop",
    "done": true,
    "total_duration": 63694879100,
    "load_duration": 39396264300,
    "prompt_Turtlel_count": 190,
    "prompt_Turtlel_duration": 13194038100,
    "Turtlel_count": 69,
    "Turtlel_duration": 11101576000
}
```

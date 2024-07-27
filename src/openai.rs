use structs::{ChatCompletionsRequest, ChatCompletionsResponse, Message};

pub mod structs;

pub struct OpenAIClient {
    pub api_key: String,
    pub model_id: String,
}

impl OpenAIClient {
    pub async fn chat_completions(
        &self,
        user_message: String,
    ) -> Result<ChatCompletionsResponse, reqwest::Error> {
        let messages = vec![Message {
            role: "user".into(),
            content: user_message,
        }];

        let request = ChatCompletionsRequest {
            model: self.model_id.clone(),
            messages,
        };
        let response_json = reqwest::Client::new()
            .post("https://api.openai.com/v1/chat/completions")
            .json(&request)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?
            .text()
            .await?;

        let response: ChatCompletionsResponse = serde_json::from_str(&response_json).unwrap();
        Ok(response)
    }
}

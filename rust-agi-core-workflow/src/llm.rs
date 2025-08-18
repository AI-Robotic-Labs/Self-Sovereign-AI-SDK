use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;
use reqwest::Client;
use crate::config::Config;

#[derive(Debug, Clone)]
pub struct LLMClient {
    http: Client,
    endpoint: String,
    api_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LLMResponse {
    pub text: String,
}

impl LLMClient {
    pub fn new(cfg: &Config) -> Self {
        Self {
            http: Client::new(),
            endpoint: cfg.llm_endpoint.clone(),
            api_key: cfg.llm_api_key.clone(),
        }
    }

    /// ask the model for a strict JSON plan; caller should craft the system/user prompt
    pub async fn send_chat(&self, system: &str, user: &str) -> Result<String> {
        // Basic OpenAI-compatible request. If you use a different provider, change this.
        let body = json!({
            "model": "gpt-4o-mini",
            "messages": [
                {"role":"system","content": system},
                {"role":"user","content": user}
            ],
            "temperature": 0.0,
            "max_tokens": 800
        });

        let mut req = self.http.post(&self.endpoint).json(&body);
        if let Some(key) = &self.api_key {
            req = req.bearer_auth(key);
        }
        let resp = req.send().await?;
        let j: serde_json::Value = resp.json().await?;
        // Try to extract a sensible text response; adapt to provider shape.
        if let Some(text) = j
            .pointer("/choices/0/message/content")
            .and_then(|v| v.as_str())
        {
            Ok(text.to_string())
        } else if let Some(text) = j.pointer("/choices/0/text").and_then(|v| v.as_str()) {
            Ok(text.to_string())
        } else {
            Ok(j.to_string())
        }
    }
}

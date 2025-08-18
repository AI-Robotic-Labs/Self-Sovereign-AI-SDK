use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    /// LLM endpoint (OpenAI or local)
    pub llm_endpoint: String,
    pub llm_api_key: Option<String>,
    pub max_plan_steps: usize,
    pub allow_shell: bool,
    // SDK-specific config (passed to SSAI init)
    pub ssai_config: Option<serde_json::Value>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            llm_endpoint: "https://api.openai.com/v1/chat/completions".to_string(),
            llm_api_key: std::env::var("OPENAI_API_KEY").ok(),
            max_plan_steps: 6,
            allow_shell: false,
            ssai_config: None,
        }
    }
}

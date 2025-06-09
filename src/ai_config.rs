use std::collections::HashMap;

pub struct AIConfig {
    pub model: String,                          // The name or type of the AI model
    pub max_tokens: u32,                        // Maximum tokens allowed for responses
    pub api_key: String,                        // API key for authentication
    pub llm_api_url: String,                    // URL for the LLM API
    pub llm_api_key: String,                    // API key for the LLM API
    pub ai_agent_id: String,                    // Identifier for the AI agent
    pub ai_agent_url: String,                   // URL or endpoint of the AI agent
    pub input_parameters: HashMap<String, String>, // Input parameters for requests
    pub output_parameters: HashMap<String, String>, // Expected output parameters
    pub max_compute: u32,                       // Maximum compute resources
    pub parameters: HashMap<String, f32>,       // Model-specific floating-point parameters
    pub local: String,                          // Local server
}

impl AIConfig {
    pub fn new(model: String, max_compute: u32, max_tokens: u32) -> Self {
        Self {
            model,
            max_compute,
            max_tokens,
            api_key: String::new(),
            llm_api_url: String::new(),
            llm_api_key: String::new(),
            ai_agent_id: String::new(),
            ai_agent_url: String::new(),
            input_parameters: HashMap::new(),
            output_parameters: HashMap::new(),
            parameters: HashMap::new(),
            local: String::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::AIConfig;

    #[test]
    fn test_ai_config() {
        let ai_config = AIConfig::new(
            "gpt-4".to_string(),  // Model name
            1000,                // Max compute resources
            2048,                // Max tokens
        );

        println!("AIConfig: {:?}", ai_config.model);
    }
}

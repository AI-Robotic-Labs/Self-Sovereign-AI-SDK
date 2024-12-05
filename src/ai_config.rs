use std::collections::HashMap;

pub struct AIConfig {
    pub model: String,
    pub max_tokens: u32,
    pub api_key: String,
    pub llm_api_url: String,
    pub llm_api_key: String,
    pub ai_agent_id: String,
    pub ai_agent_url: String,
    pub input_parameters: HashMap<String, String>,
    pub output_parameters: HashMap<String, String>,
    pub max_compute: u32,
    pub parameters: HashMap<String, f32>,
}

impl AIConfig {
    pub fn new(model: String, max_compute: u32) -> Self {
        Self {
            model,
            max_compute,
            parameters: HashMap::new(),
            input_parameters: HashMap::new(),
            output_parameters: HashMap::new(),
            api_key: String::new(),
            llm_api_url: String::new(),
            llm_api_key: String::new(),
            ai_agent_id: String::new(),
            ai_agent_url: String::new(),
            max_tokens: 0,

        }
    }
}

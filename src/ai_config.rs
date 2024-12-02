use std::collections::HashMap;

pub struct AIConfig {
    pub model: String,
    pub max_compute: u32,
    pub parameters: HashMap<String, f32>,
}

impl AIConfig {
    pub fn new(model: String, max_compute: u32) -> Self {
        Self {
            model,
            max_compute,
            parameters: HashMap::new(),
        }
    }
}

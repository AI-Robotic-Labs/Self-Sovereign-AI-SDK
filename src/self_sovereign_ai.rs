use crate::{AIConfig, DHT, NetworkConfig};
use std::collections::HashMap;

pub struct SelfSovereignAI {
    pub node_id: String,
    pub dht: DHT,
    pub ai_config: AIConfig,
    pub local_data: HashMap<String, String>,
    pub network: NetworkConfig,
    pub ai_agent_id: String,
    pub api_token: String,
    pub llm_identifier: Option<String>,
    pub llm_api_key: Option<String>,
}

impl SelfSovereignAI {
    pub fn new(node_id: String, ai_config: AIConfig, network: NetworkConfig) -> Self {
        Self {
            node_id,
            dht: DHT::new(),
            ai_config,
            local_data: HashMap::new(),
            network,
            ai_agent_id: String::new(),
            api_token: String::new(),
            llm_identifier: None,
            llm_api_key: None,
        }
    }

    pub fn store_in_dht(&mut self, key: String, value: String) {
        self.dht.store(key, value);
    }

    pub fn retrieve_from_dht(&self, key: &str) -> Option<&String> {
        self.dht.retrieve(key)
    }

    pub fn add_peer(&mut self, peer_address: String) {
        self.dht.add_peer(peer_address);
    }
}

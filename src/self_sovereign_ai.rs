use std::collections::HashMap;

use crate::DHT;
use crate::NetworkConfig;
use crate::AIConfig;

pub struct SelfSovereignAI {
    pub node_id: String,                        // Unique identifier for the node
    pub dht: DHT,                               // Distributed Hash Table instance
    pub ai_config: AIConfig,                    // AI configuration settings
    pub local_data: HashMap<String, String>,    // Local data storage
    pub network: NetworkConfig,                 // Network configuration
    pub ai_agent_id: String,                    // Identifier for the AI agent
    pub ai_agent_url: String,                   // URL of the AI agent endpoint
    pub api_token: String,                      // Token for API authentication
    pub llm_identifier: Option<String>,         // Optional identifier for the LLM
    pub llm_api_key: Option<String>,            // Optional API key for the LLM
    pub local_server: String,                   // Local server URL
}

impl SelfSovereignAI {
    /// Constructor to create a new SelfSovereignAI instance
    pub fn new(node_id: String, ai_config: AIConfig, network: NetworkConfig) -> Self {
        Self {
            node_id,
            dht: DHT::new(),
            ai_config,
            local_data: HashMap::new(),
            network,
            ai_agent_id: String::new(),
            ai_agent_url: String::new(),
            api_token: String::new(),
            llm_identifier: None,
            llm_api_key: None,
            local_server: String::new(),
        }
    }

    /// Store a key-value pair in the DHT
    pub fn store_in_dht(&mut self, key: String, value: String) {
        self.dht.store(key, value);
    }

    /// Retrieve a value from the DHT by its key
    pub fn retrieve_from_dht(&self, key: &str) -> Option<&String> {
        self.dht.retrieve(key)
    }

    /// Add a peer to the DHT
    pub fn add_peer(&mut self, peer_address: String) {
        self.dht.add_peer(peer_address);
    }
}

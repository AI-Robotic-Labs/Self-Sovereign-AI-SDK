// Declare modules
pub mod ai_config;
pub mod dht;
pub mod network;
pub mod self_sovereign_ai;

// Re-export the core struct
pub use self_sovereign_ai::SelfSovereignAI;
pub use ai_config::AIConfig;
pub use dht::DHT;
pub use network::NetworkConfig;

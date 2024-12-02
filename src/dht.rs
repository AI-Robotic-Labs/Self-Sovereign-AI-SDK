use std::collections::HashMap;

pub struct DHT {
    pub local_storage: HashMap<String, String>,
    pub peer_nodes: Vec<String>,
}

impl DHT {
    pub fn new() -> Self {
        Self {
            local_storage: HashMap::new(),
            peer_nodes: Vec::new(),
        }
    }

    pub fn store(&mut self, key: String, value: String) {
        self.local_storage.insert(key, value);
    }

    pub fn retrieve(&self, key: &str) -> Option<&String> {
        self.local_storage.get(key)
    }

    pub fn add_peer(&mut self, peer_address: String) {
        if !self.peer_nodes.contains(&peer_address) {
            self.peer_nodes.push(peer_address);
        }
    }
}

pub struct NetworkConfig {
    pub address: String,
    pub port: u16,
    pub protocol: String,
    pub local_ip: String,
}

impl NetworkConfig {
    pub fn new(address: String, port: u16, protocol: String) -> Self {
        Self {
            address,
            port,
            protocol,
            local_ip: String::new(),
        }
    }
}

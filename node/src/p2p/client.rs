pub struct Client {
    pub address: String,
    pub peers: Vec<String>
}

impl Client {
    pub fn new(address: String) -> Client {
        Self {
            address,
            peers: Vec::new()
        }
    }

    pub fn add_peer(&mut self, address: String) {
        self.peers.push(address);
    }
}
use std::time::SystemTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Message {
    PING { timestamp: u64 },
    PONG { timestamp: u64 },
}

impl Message {
    pub fn create_ping() -> String {
        let timestamp = Self::get_timestamp();
        let message = Message::PING { timestamp };

        message.serialize()
    }

    pub fn create_pong() -> String {
        let timestamp = Self::get_timestamp();
        let message = Message::PONG { timestamp };

        message.serialize()
    }

    fn get_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
use std::cell::RefCell;
use std::rc::Rc;
use chrono::DateTime;
use hex::encode;
use sha2::{Digest, Sha256};
use crate::simulator::log_panel::LogPanel;

pub type SharedLogPanel = Rc<RefCell<LogPanel>>;

pub fn calculate_hash(serialized_data: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(serialized_data.as_bytes());
    let result = hasher.finalize();

    encode(result)
}

pub fn format_timestamp(timestamp: i64) -> String {
    let datetime = DateTime::from_timestamp(timestamp, 0).unwrap();
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}
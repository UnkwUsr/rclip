use serde::{Deserialize, Serialize};

// TODO: store images in separate file
#[derive(Serialize, Deserialize, Debug)]
pub struct HistoryEntry {
    pub buf: Vec<u8>,
    target_name: String,
}

impl HistoryEntry {
    pub fn new(buf: Vec<u8>, target_name: String) -> Self {
        HistoryEntry { buf, target_name }
    }
}

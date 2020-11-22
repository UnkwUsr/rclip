use serde::{Deserialize, Serialize};

// TODO: store images in separate file
#[derive(Serialize, Deserialize, Debug)]
pub struct HistoryEntry {
    pub buf: Vec<u8>,
}

impl HistoryEntry {
    pub fn new(buf: Vec<u8>) -> Self {
        HistoryEntry { buf }
    }
}

mod entry;
pub use entry::HistoryEntry;

use serde::{Deserialize, Serialize};
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct History(Vec<HistoryEntry>);

impl History {
    pub fn from_file(filename: &str) -> Self {
        match std::fs::File::open(filename) {
            Ok(mut f) => {
                let mut binencoded = Vec::new();
                f.read_to_end(&mut binencoded).unwrap();
                match bincode::deserialize(&binencoded) {
                    Ok(history) => History(history),
                    // TODO: should expand error correct, for possible to print text of error.
                    // Now this not work
                    Err(e) => panic!(e),
                }
            }
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => {
                    println!("Clipboard history file does not exist.");
                    History(Vec::new())
                }
                e => panic!(e),
            },
        }
    }

    pub fn write_file(&self, filename: &str) {
        let encoded: Vec<u8> = bincode::serialize(&self.0).unwrap();
        let mut f = std::fs::File::create(filename).unwrap();
        f.write_all(&encoded).unwrap();
    }

    pub fn push(&mut self, entry: HistoryEntry) {
        self.0.push(entry);
    }

    pub fn get_last_entry(&self) -> Option<&HistoryEntry> {
        self.0.last()
    }
}

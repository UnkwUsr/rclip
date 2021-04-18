mod entry;
pub use entry::HistoryEntry;
use std::io::BufReader;

pub struct History(Vec<HistoryEntry>);

impl History {
    pub fn from_file(filename: &str) -> Self {
        let mut v: Vec<HistoryEntry> = Vec::new();
        let f = std::fs::OpenOptions::new()
            .read(true)
            // not found error can't happen because we create that file in module Paths
            .open(filename)
            .unwrap();

        let mut bufreader = BufReader::new(f);
        while let Some(entry) = HistoryEntry::next_from_bufreader(&mut bufreader) {
            v.push(entry);
        }

        History(v)
    }

    pub fn print(&self) {
        for (i, x) in self.0.iter().enumerate() {
            // use alerante format for entry for print all text in one line
            println!("{} {:#}", i, x);
        }
    }

    pub fn print_by_id(&self, id: usize) -> Result<(), ()> {
        if id > self.0.len() {
            Err(())
        } else {
            eprint!("{}", self.0[id]);
            Ok(())
        }
    }
}

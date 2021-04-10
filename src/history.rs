mod entry;
pub use entry::HistoryEntry;
use std::io::BufReader;

pub struct History(Vec<HistoryEntry>);

impl History {
    pub fn from_file(filename: &str) -> Self {
        let mut v: Vec<HistoryEntry> = Vec::new();
        let f = std::fs::OpenOptions::new()
            .read(true)
            .open(filename)
            .unwrap();

        let mut bufreader = BufReader::new(f);
        while let Some(entry) = HistoryEntry::read_next(&mut bufreader) {
            v.push(entry);
        }

        History(v)
    }

    pub fn print(&self) {
        for (i, x) in self.0.iter().enumerate() {
            println!("{} {}", i, x);
        }

    }

    pub fn print_by_id(&self, id: usize) {
        eprintln!("{}", self.0[id]);
    }
}


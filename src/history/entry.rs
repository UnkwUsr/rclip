use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

pub struct HistoryEntry {
    pub buf: Vec<u8>,
    pub target_name: String,
}

impl HistoryEntry {
    // pub fn new(buf: Vec<u8>, target_name: String) -> Self {
    //     HistoryEntry { buf, target_name }
    // }

    pub fn read_next(bufreader: &mut BufReader<File>) -> Option<Self> {
        let mut target_name = String::new();
        bufreader.read_line(&mut target_name).unwrap();
        if target_name.is_empty() {
            None
        } else {
            let mut len_str = String::new();
            bufreader.read_line(&mut len_str).unwrap();
            // println!("{:?}", len_str.trim());
            let len = len_str.trim().parse::<usize>().unwrap();

            let mut buf = vec![0; len];
            bufreader.read_exact(&mut buf).unwrap();
            let mut trailing_new_line = String::new();
            bufreader.read_line(&mut trailing_new_line).unwrap();
            // println!("{:?}", buf);

            Some(HistoryEntry { buf, target_name })
        }
    }
}

impl std::fmt::Display for HistoryEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        let mut v = self.buf.clone();
        for x in &mut v {
            if *x == b'\n' {
                *x = b' '
            }
        }
        write!(f, "{}", String::from_utf8(v).unwrap())?;

        Ok(())
    }
}

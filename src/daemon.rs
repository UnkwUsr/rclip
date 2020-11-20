use crate::clipboard::Getter;

use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;

pub struct Daemon<'a> {
    getter: Getter<'a>,
    last_buf: Vec<u8>,
    file: File,
}

impl<'a> Daemon<'a> {
    pub fn new() -> Self {
        let getter = Getter::new();
        let last_buf = Vec::new();
        let file = File::create("outasd.bin").unwrap();

        Daemon {
            getter,
            last_buf,
            file,
        }
    }

    pub fn start_loop(&mut self) {
        loop {
            std::thread::sleep(::std::time::Duration::from_millis(100));

            let mut new_buf = Vec::new();
            self.getter.get(&mut new_buf);

            if new_buf == self.last_buf {
                continue;
            }

            self.file.set_len(0).unwrap();
            self.file.seek(SeekFrom::Start(0)).unwrap();
            self.file.write_all(&new_buf).unwrap();
            // self.file.sync_all().unwrap();
            println!(
                "Clipboard changed. Len: {}. Writed to file outasd.bin",
                new_buf.len()
            );

            self.last_buf = new_buf;
        }
    }
}

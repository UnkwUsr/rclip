use crate::clipboard::ClipboardCtx;
use crate::clipboard::Getter;
use std::io::Write;
use std::path::Path;
use std::time::SystemTime;

use signal_hook::{iterator::Signals, SIGUSR1};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;

const HISTORY_DIR: &str = "rclips";

pub struct Daemon<'a> {
    getter: Getter<'a>,
}

impl<'a> Daemon<'a> {
    pub fn new(clipboard_ctx: &'a ClipboardCtx) -> Self {
        let getter = Getter::new(&clipboard_ctx);
        std::fs::create_dir_all(HISTORY_DIR).unwrap();

        Daemon { getter }
    }

    pub fn start_loop(&mut self) {
        let dooneskip = Arc::new(AtomicBool::new(false));
        let dooneskip_shared = dooneskip.clone();

        let signals = Signals::new(&[SIGUSR1]).unwrap();
        std::thread::spawn(move || {
            for _ in signals.forever() {
                dooneskip_shared.store(true, Ordering::Release);
            }
        });

        loop {
            std::thread::sleep(::std::time::Duration::from_millis(100));

            let mut new_buf = Vec::new();
            match self.getter.get_wait(&mut new_buf) {
                Ok(target_name) => {
                    // TODO: search in folder to find if we already have that entry
                    // if let Some(prev_entry) = self.history.get_last_entry() {
                    //     if new_buf == prev_entry.buf {
                    //         continue;
                    //     }
                    // }

                    if dooneskip.load(Ordering::Relaxed) {
                        println!("skip due to 'set from history'");
                        dooneskip.store(false, Ordering::Release);
                        continue;
                    }

                    // TODO: add config setting 'length of minimum clip'
                    if new_buf.is_empty() {
                        println!("skip due to 'too short'");
                        continue;
                    }

                    println!("Clipboard changed. Len: {}", new_buf.len());

                    let file_name = format!(
                        "{}/{}.{}",
                        HISTORY_DIR,
                        SystemTime::now()
                            .duration_since(SystemTime::UNIX_EPOCH)
                            .unwrap()
                            .as_millis(),
                        // replace due to we can't use this symbol in file name
                        target_name.replace("/", "\\")
                    );
                    let file_path = Path::new(&file_name);

                    let mut f = std::fs::File::create(file_path).unwrap();
                    f.write_all(&new_buf).unwrap();
                }
                Err(()) => continue,
            };
        }
    }
}

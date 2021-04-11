use crate::clipboard::ClipboardCtx;
use crate::clipboard::Getter;
use signal_hook::{iterator::Signals, SIGUSR1};
use std::io::Write;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

pub const HISTORY_FILE: &str = "history.rclips";

pub struct Daemon<'a> {
    getter: Getter<'a>,
}

impl<'a> Daemon<'a> {
    pub fn new(clipboard_ctx: &'a ClipboardCtx) -> Self {
        let getter = Getter::new(&clipboard_ctx);

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

                    // TODO: do check for non-text target
                    if target_name == "image/png" {
                        let filepathstring = format!(
                            "by_target_name/{}/{}",
                            target_name,
                            SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .unwrap()
                                .as_millis(),
                        );
                        let filepath = std::path::Path::new(&filepathstring);
                        std::fs::create_dir_all(filepath.parent().unwrap()).unwrap();
                        let mut f = std::fs::OpenOptions::new()
                            .write(true)
                            .create(true)
                            .open(filepath)
                            .unwrap();
                        f.write_all(&new_buf).unwrap();
                        // TODO: write entry about this image to general history file
                    } else {
                        let mut f = std::fs::OpenOptions::new()
                            .append(true)
                            .create(true)
                            .open(HISTORY_FILE)
                            .unwrap();
                        f.write_all(target_name.as_bytes()).unwrap();
                        f.write_all(&[b'\n']).unwrap();
                        f.write_all(new_buf.len().to_string().as_bytes()).unwrap();
                        f.write_all(&[b'\n']).unwrap();
                        f.write_all(&new_buf).unwrap();
                        f.write_all(&[b'\n']).unwrap();
                    }
                }
                Err(()) => continue,
            };
        }
    }
}

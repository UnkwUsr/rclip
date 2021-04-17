use crate::clipboard::ClipboardCtx;
use crate::clipboard::Getter;
use crate::clipboard::GetterError;
use crate::config::Config;
use crate::Paths;
use signal_hook::{iterator::Signals, SIGUSR1};
use std::io::Write;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Daemon<'a> {
    getter: Getter<'a>,
    paths: &'a Paths,
    config: &'a Config,
}

impl<'a> Daemon<'a> {
    pub fn new(config: &'a Config, paths: &'a Paths, clipboard_ctx: &'a ClipboardCtx) -> Self {
        let getter = Getter::new(config, &clipboard_ctx);

        Daemon {
            config,
            getter,
            paths,
        }
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
                    // TODO: check for duplicates

                    if dooneskip.load(Ordering::Relaxed) {
                        println!("skip due to 'set from history'");
                        dooneskip.store(false, Ordering::Release);
                        continue;
                    }

                    if new_buf.len() < self.config.min_length {
                        println!("skip due to 'too short'");
                        continue;
                    }

                    println!("Clipboard changed. Len: {}", new_buf.len());

                    let mut history_file = std::fs::OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(&self.paths.history_file_path)
                        .unwrap();

                    if self.config.other_targets.contains(&target_name) {
                        let filename = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_millis()
                            .to_string();
                        let filepathstring = format!(
                            "{}/{}/{}",
                            self.paths.other_targets_dir_path, target_name, filename,
                        );
                        let filepath = std::path::Path::new(&filepathstring);
                        let mut f = std::fs::OpenOptions::new()
                            .write(true)
                            .create(true)
                            .open(filepath)
                            .unwrap();
                        f.write_all(&new_buf).unwrap();

                        history_file
                            .write_all(
                                format!(
                                    "{} {} {}\n{}",
                                    target_name,
                                    filename.len(),
                                    "!",
                                    filename.to_string()
                                )
                                .as_bytes(),
                            )
                            .unwrap();
                    } else {
                        history_file
                            .write_all(format!("{} {}\n", target_name, new_buf.len()).as_bytes())
                            .unwrap();
                        history_file.write_all(&new_buf).unwrap();
                    }

                    // print new line after history entry
                    history_file.write_all(&[b'\n']).unwrap();
                }
                Err(GetterError::UnknownTarget) => {
                    println!("Unknown target. Check settings 'text_targets' and 'other_targets' in config.")
                }
            };
        }
    }
}

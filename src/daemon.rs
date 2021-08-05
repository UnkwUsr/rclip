use crate::clipboard::ClipboardCtx;
use crate::clipboard::Getter;
use crate::clipboard::GetterError;
use crate::config::Config;
use crate::utils::get_hash;
use crate::Paths;
use signal_hook::{consts::signal::SIGUSR1, iterator::Signals};
use std::collections::hash_map::DefaultHasher;
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
            getter,
            paths,
            config,
        }
    }

    pub fn start_loop(&mut self) {
        let dooneskip = Arc::new(AtomicBool::new(false));
        let dooneskip_shared = dooneskip.clone();

        let mut signals = Signals::new(&[SIGUSR1]).unwrap();
        std::thread::spawn(move || {
            for _ in signals.forever() {
                dooneskip_shared.store(true, Ordering::Release);
            }
        });

        let mut prev_hash: u64 = 0;
        loop {
            std::thread::sleep(::std::time::Duration::from_millis(100));

            let mut clipboard_data = Vec::new();
            match self.getter.get_wait(&mut clipboard_data) {
                Ok(target_name) => {
                    if dooneskip.load(Ordering::Relaxed) {
                        println!("[rclip] Skip because of got signal for skip");
                        dooneskip.store(false, Ordering::Release);
                        continue;
                    }

                    if clipboard_data.len() < self.config.min_length {
                        println!("[rclip] Skip due to config setting 'min_length'");
                        continue;
                    }

                    let mut hasher = DefaultHasher::new();
                    let new_hash = get_hash(&clipboard_data, &mut hasher);
                    if new_hash == prev_hash {
                        println!("[rclip] Found duplicate");
                        continue;
                    }
                    prev_hash = new_hash;

                    println!("[rclip] Clipboard changed. Len: {}", clipboard_data.len());

                    let filename = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis()
                        .to_string();
                    let filepathstring = format!(
                        "{}/{}/{}",
                        self.paths.history_dir_path, target_name, filename,
                    );
                    let filepath = std::path::Path::new(&filepathstring);
                    let mut f = std::fs::OpenOptions::new()
                        .write(true)
                        .create(true)
                        .open(filepath)
                        .unwrap();
                    f.write_all(&clipboard_data).unwrap();
                }
                Err(GetterError::UnknownTarget) => {
                    eprintln!("[rclip] Unknown target. Check setting 'targets_list' in config.")
                }
            };
        }
    }
}

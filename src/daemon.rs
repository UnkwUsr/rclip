use crate::clipboard::ClipboardCtx;
use crate::clipboard::Getter;
use crate::clipboard::Setter;
use crate::history::History;
use crate::history::HistoryEntry;

const HISTORY_FILENAME: &str = "history.rclip";

// TODO: maybe open file only once and then use them for write?
// instead of opening each time before write
pub struct Daemon<'a> {
    getter: Getter<'a>,
    setter: Setter<'a>,
    // file: File,
    history: History,
}

impl<'a> Daemon<'a> {
    pub fn new(clipboard_ctx: &'a ClipboardCtx) -> Self {
        let getter = Getter::new(&clipboard_ctx);
        let setter = Setter::new(&clipboard_ctx);
        let history = History::from_file(HISTORY_FILENAME);

        Daemon {
            getter,
            setter,
            // file,
            history,
        }
    }

    pub fn start_loop(&mut self) {
        loop {
            std::thread::sleep(::std::time::Duration::from_millis(100));

            let mut new_buf = Vec::new();
            let target_name = self.getter.get_wait(&mut new_buf);

            if let Some(prev_entry) = self.history.get_last_entry() {
                if new_buf == prev_entry.buf {
                    continue;
                }
            }

            println!("Clipboard changed. Len: {}", new_buf.len());

            let history_entry = HistoryEntry::new(new_buf, target_name);
            self.history.push(history_entry);

            self.history.write_file(HISTORY_FILENAME);
        }
    }
}

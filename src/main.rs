pub mod clipboard;
pub mod history;

mod daemon;

use clipboard::ClipboardCtx;
use daemon::Daemon;

fn main() {
    let clipboard_ctx = ClipboardCtx::new();
    let mut daemon = Daemon::new(&clipboard_ctx);
    daemon.start_loop();
}

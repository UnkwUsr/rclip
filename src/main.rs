pub mod clipboard;
pub mod history;

mod daemon;
use daemon::Daemon;

fn main() {
    let mut daemon = Daemon::new();
    daemon.start_loop();
}

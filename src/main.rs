use clap::{App, Arg, SubCommand};

pub mod clipboard;
pub mod history;

mod daemon;

use clipboard::ClipboardCtx;
use daemon::Daemon;

fn main() {
    let arg_matches = App::new("rclip")
        .version("0.1.0")
        .author("UnkwUsr <ktoto2707043 at gmail dot cum>")
        .about("Clipboard manager written in Rust")
        .subcommand(SubCommand::with_name("daemon").about("Run daemon of clipboard manager"))
        .subcommand(SubCommand::with_name("list").about("Print list of all entries in history"))
        .subcommand(
            SubCommand::with_name("set")
                .about("Set picked entry as current clipboard")
                .arg(
                    Arg::with_name("entry index")
                        .short("i")
                        .long("index")
                        .help("Index of wanted entry to set. Find it in `list`")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .get_matches();

    match arg_matches.subcommand() {
        ("daemon", Some(_)) => {
            let clipboard_ctx = ClipboardCtx::new();
            let mut daemon = Daemon::new(&clipboard_ctx);
            daemon.start_loop();
        }
        ("list", Some(_)) => {
            unimplemented!("list")
        }
        ("set", Some(_)) => {
            unimplemented!("set")
        }
        _ => {
            unimplemented!("'help' must be here")
        }
    }
}

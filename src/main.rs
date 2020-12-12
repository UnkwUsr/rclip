use clap::{App, SubCommand};

pub mod clipboard;

mod daemon;

use clipboard::ClipboardCtx;
use daemon::Daemon;

fn main() {
    // TODO: detect if another program instanec already launched
    let arg_matches = App::new("rclip")
        .version("0.1.0")
        .author("UnkwUsr <ktoto2707043 at gmail dot cum>")
        .about("Clipboard manager written in Rust")
        // .arg(Arg::with_name("daemon").short("d").long("daemon").help("Run daemon of clipboard manager"))
        .subcommand(SubCommand::with_name("daemon").about("Run daemon of clipboard manager"))
        .get_matches();

    match arg_matches.subcommand() {
        ("daemon", Some(_)) => {
            let clipboard_ctx = ClipboardCtx::new();
            let mut daemon = Daemon::new(&clipboard_ctx);
            daemon.start_loop();
        }
        _ => {
            println!("{}", arg_matches.usage());
        }
    }
}

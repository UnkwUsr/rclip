use clap::{App, SubCommand};

mod clipboard;
mod config;
mod daemon;
mod paths;
mod utils;

use clipboard::ClipboardCtx;
use config::Config;
use daemon::Daemon;
use paths::Paths;

fn main() {
    // TODO: detect if another program instanec already launched
    let arg_matches = App::new("rclip")
        .version("0.1.0")
        .author("UnkwUsr <ktoto2707043 at gmail dot cum>")
        .about("Clipboard manager written in Rust")
        .subcommand(SubCommand::with_name("daemon").about("Run daemon of clipboard manager"))
        .get_matches();

    let paths = Paths::new();
    let config = Config::new(&paths);
    paths.create_targets_dirs(&config);

    match arg_matches.subcommand() {
        ("daemon", Some(_)) => {
            let clipboard_ctx = ClipboardCtx::new();
            let mut daemon = Daemon::new(&config, &paths, &clipboard_ctx);
            daemon.start_loop();
        }
        _ => {
            println!("{}", arg_matches.usage());
        }
    }
}

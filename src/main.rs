use clap::{App, SubCommand};
use std::io::Read;

mod clipboard;
mod config;
mod daemon;
mod history;
mod paths;

use clipboard::ClipboardCtx;
use config::Config;
use daemon::Daemon;
use history::History;
use paths::Paths;

fn main() {
    // TODO: detect if another program instanec already launched
    let arg_matches = App::new("rclip")
        .version("0.1.0")
        .author("UnkwUsr <ktoto2707043 at gmail dot cum>")
        .about("Clipboard manager written in Rust")
        // .arg(Arg::with_name("daemon").short("d").long("daemon").help("Run daemon of clipboard manager"))
        .subcommand(SubCommand::with_name("daemon").about("Run daemon of clipboard manager"))
        .subcommand(
            SubCommand::with_name("list_and_set")
                .about("Print list of clips and then set picked by id"),
        )
        .get_matches();

    let paths = Paths::new();
    let config = Config::new(&paths);
    paths.create_other_targets_dirs(&config);

    match arg_matches.subcommand() {
        ("daemon", Some(_)) => {
            let clipboard_ctx = ClipboardCtx::new();
            let mut daemon = Daemon::new(&config, &paths, &clipboard_ctx);
            daemon.start_loop();
        }
        ("list_and_set", Some(_)) => {
            let history = History::from_file(&paths.history_file_path);
            history.print();

            let mut buf = String::new();
            // read until space or new line
            for xa in std::io::stdin().bytes() {
                if let Ok(x) = xa {
                    if x as char == ' ' || x as char == '\n' {
                        break;
                    }
                    buf.push(x as char);
                } else {
                    break;
                }
            }
            // println!("{}", buf);
            let resi: usize = buf.parse().unwrap();
            history.print_by_id(resi);
        }
        _ => {
            println!("{}", arg_matches.usage());
        }
    }
}

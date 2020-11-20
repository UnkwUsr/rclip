mod get_clip;
mod targets;
pub use targets::Targets;
mod utils;
pub use utils::intern_atom;

use std::fs::File;
use std::io::prelude::*;

use get_clip::Getter;

fn main() {
    let mut getter = Getter::new();

    let mut prev_buf = Vec::new();
    getter.get(&mut prev_buf);

    let mut file = File::create("outasd.bin").unwrap();
    file.write_all(&prev_buf).unwrap();

    // loop {
    //     std::thread::sleep(::std::time::Duration::from_millis(100));

    //     let mut new_buf = Vec::new();
    //     getter.get(&mut new_buf);

    //     if new_buf == prev_buf {
    //         continue
    //     }

    //     println!("asd: {:?}", new_buf);
    //     prev_buf = new_buf;
    // }
}

extern crate slug;

use std::error::Error;
use std::io::{self, Read, Write};

fn normalize<S: AsRef<str>>(s: S) -> String {
    slug::slugify(s).replace('-', "")
}

fn main() {
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Cannot read from stdin");
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for s in buffer.lines() {
        let _ = handle
            .write(normalize(s).as_bytes())
            .and_then(|_| handle.write(b"\n"))
            .map_err(|e| eprintln!("{}", e.description()));
    }
}

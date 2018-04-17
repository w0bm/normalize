extern crate slug;

use std::error::Error;
use std::io::{self, BufRead, Write};

fn normalize<S: AsRef<str>>(s: S) -> String {
    slug::slugify(s).replace('-', "")
}

fn main() {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let stdin = io::stdin();

    for s in stdin
        .lock()
        .lines()
        .filter(Result::is_ok)
        .map(Result::unwrap)
    {
        let _ = handle
            .write(normalize(s).as_bytes())
            .and_then(|_| handle.write(b"\n"))
            .and_then(|_| handle.flush()) // flush stdout
            .map_err(|e| eprintln!("{}", e.description()));
    }
}

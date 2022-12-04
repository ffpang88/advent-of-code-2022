use clap::Parser;
use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead};
use std::fmt::Debug;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    filepath: String
}

pub fn cli_read_file_by_line<F>(mut f: F) where
    F: FnMut(String) {
    let args = Args::parse();

    let path = Path::new(&args.filepath);
    let display = path.display();

    let file = match File::open(&path) {
        Ok(file) => file,
        Err(reason) => panic!("couldn't open {}: {}", display, reason),
    };

    for line in BufReader::new(file).lines() {
        let string = line.unwrap();
        f(string);
    }
}

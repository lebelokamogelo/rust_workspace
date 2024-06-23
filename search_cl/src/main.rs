// TODO: grep command-line
// Your program should read a file and a search term (or regular expression) as input,
// then output all lines from the file that contain the search term.
//
use clap::Parser;
use std::{
    fs,
    io::{BufRead, BufReader},
    process,
};

#[derive(Parser, Debug)]
struct Args {
    word: String,
    file: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();

    search(args.word, args.file)
}

fn search(word: String, file: std::path::PathBuf) {
    let buffer = BufReader::new(match fs::File::open(file) {
        Ok(file) => file,
        Err(err) => {
            println!("{}", err);
            process::exit(0)
        }
    });
    for line in buffer.lines() {
        let line = line.unwrap();
        if line.to_lowercase().contains(word.to_lowercase().as_str()) {
            println!("- {}", line)
        }
    }
}

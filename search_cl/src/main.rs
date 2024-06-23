// TODO: grep command-line
// Your program should read a file and a search term (or regular expression) as input,
// then output all lines from the file that contain the search term.
//
use clap::Parser;
use std::fs;

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
    let content = fs::read_to_string(file).unwrap();

    for line in content.lines() {
        if line.to_lowercase().contains(word.to_lowercase().as_str()) {
            println!("- {}", line)
        }
    }
}

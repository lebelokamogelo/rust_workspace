// TODO: grep command-line
// Your program should read a file and a search term (or regular expression) as input,
// then output all lines from the file that contain the search term.
//
use regex::Regex;
use std::env;
use std::fs;

struct Arguments<'a> {
    file: &'a str,
    word: &'a str,
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 3 {
        if args.len() == 2 && args[1] == "help" {
            println!("Usage: cargo run [FILE] [WORD] [OPTIONS]");
            println!("Options:");
            println!("help display this help and exit\n");
        } else {
            eprintln!("error: invalid arguments\nTry: cargo run help");
        }
        return;
    }

    let file = &args[1];
    let word = &args[2];

    search(Arguments { file, word })
}

fn search(args: Arguments) {
    let content = fs::read_to_string(args.file).unwrap();

    let pattern = Regex::new(format!("(?i){}", args.word).as_ref()).unwrap();

    content
        .lines()
        .map(|line| match pattern.find(line) {
            Some(_) => line,
            None => "",
        })
        .filter(|line| !line.is_empty())
        .for_each(|line| println!("- {line}"));
}

use std::{
    collections::HashMap,
    io::{self, Write},
    process,
};

use rand::{self, distributions::Alphanumeric, Rng};

fn random_identifier() -> String {
    let identifier: Vec<_> = rand::thread_rng()
        .sample_iter(Alphanumeric)
        .take(8)
        .collect();

    String::from_utf8_lossy(&identifier[..]).to_string()
}

fn main() {
    let mut urls: HashMap<String, String> = HashMap::new();

    loop {
        println!("1. Url shortener");
        println!("2. Search...");
        println!("3. Exit");
        print!(">> ");

        let _ = std::io::stdout().flush();

        let mut choice = String::new();

        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read the line");

        let choice = match choice.trim().parse::<u32>() {
            Err(_) => continue,
            Ok(choice) => choice,
        };

        match choice {
            1 => {
                let mut url = String::new();

                print!("Enter the url to shorten: ");
                let _ = std::io::stdout().flush();

                io::stdin()
                    .read_line(&mut url)
                    .expect("Failed to read a line");

                let shortened_url = format!("https://url.ly/{}/", random_identifier());
                println!("You new url is {shortened_url}");

                urls.insert(shortened_url, url);
            }
            2 => {
                let mut search_url = String::new();

                print!("Search with url shortener: ");
                let _ = std::io::stdout().flush();

                io::stdin()
                    .read_line(&mut search_url)
                    .expect("Failed to read a line");

                match urls.get(search_url.trim()) {
                    Some(url) => println!("Redirecting to {url}\n"),
                    None => println!("We’re having trouble finding that site.\n"),
                }
            }
            3 => process::exit(0),
            _ => println!("Invalid option: Try again\n"),
        }
    }
}

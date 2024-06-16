use reqwest;
use serde::Deserialize;

// parse the json into struct rust redable format
#[derive(Deserialize, Debug)]
struct Post {
    userId: i32,
    id: i32,
    title: String,
    body: String,
}

fn main() {
    // blocking client
    let response = reqwest::blocking::get("https://jsonplaceholder.typicode.com/posts").unwrap();

    // matching status code
    match response.status() {
        reqwest::StatusCode::OK => {
            // process data
            let posts: Vec<Post> = serde_json::from_str(&response.text().unwrap()).unwrap();
            println!("{:?}", posts[0].title)
        }

        reqwest::StatusCode::UNAUTHORIZED => println!("You are not authenticated"),
        _ => print!("Uh oh! Something went wrong"),
    }
}

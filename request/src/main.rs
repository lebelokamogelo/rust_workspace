use reqwest;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

// parse the json into struct rust redable format
#[derive(Deserialize, Serialize, Debug)]
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
            let _posts: Vec<Post> = serde_json::from_str(&response.text().unwrap()).unwrap();
            // println!("{:?}", posts[0].title)
        }

        reqwest::StatusCode::UNAUTHORIZED => println!("You are not authenticated"),
        _ => print!("Uh oh! Something went wrong"),
    }

    // post request
    // create the blocking request client
    let client = Client::new();

    let data = Post {
        userId: 1,
        id: 101,
        title: "New title".to_owned(),
        body: "New body".to_owned(),
    };

    // sending the request
    let response = client
        .post("https://jsonplaceholder.typicode.com/posts")
        .body(serde_json::to_string(&data).unwrap())
        .header("Content-Type", "application/json")
        .send()
        .unwrap();

    let response_data: Post = serde_json::from_str(&response.text().unwrap()).unwrap();

    println!("{:?}", response_data)
}

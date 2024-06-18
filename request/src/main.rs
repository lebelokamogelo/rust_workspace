#![allow(non_snake_case)]

use reqwest;
use serde::{Deserialize, Serialize};

// parse the json into struct rust redable format
#[derive(Deserialize, Serialize, Debug)]
struct Post {
    userId: i32,
    id: i32,
    title: String,
    body: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // client
    let client = reqwest::Client::new();

    let response = client
        .get("https://jsonplaceholder.typicode.com/posts")
        .send()
        .await
        .unwrap();

    // matching status code
    match response.status() {
        reqwest::StatusCode::OK => {
            // process data
            let _posts: Vec<Post> = serde_json::from_str(&response.text().await.unwrap()).unwrap();
            // println!("{:?}", posts[0].title)
        }

        reqwest::StatusCode::UNAUTHORIZED => println!("You are not authenticated"),
        _ => print!("Uh oh! Something went wrong"),
    }

    // post request

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
        .await
        .unwrap();

    let response_data: Post = serde_json::from_str(&response.text().await.unwrap()).unwrap();

    println!("{:?}", response_data);
    Ok(())
}

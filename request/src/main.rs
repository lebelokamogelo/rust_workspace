use reqwest;

fn main() {
    let response = reqwest::blocking::get("https://jsonplaceholder.typicode.com/users").unwrap();

    let users = response.text();
    print!("{:?}", users)
}

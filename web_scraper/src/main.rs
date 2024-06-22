use reqwest;
use select::{document::Document, predicate::Name};

fn main() {
    let response =
        reqwest::blocking::get("https://supersport.com/football/uefa-euro/top-scorers").unwrap();

    let document = Document::from(response.text().unwrap().as_str());
    for tr in document.find(Name("tr")) {
        for td in tr.find(Name("td")) {
            println!("{}", td.text())
        }
        println!("\n")
    }
}

use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut url = String::new();

    print!("Enter the image url to download: ");
    _ = io::stdout().flush();

    io::stdin().read_line(&mut url).unwrap();

    let mut response = reqwest::blocking::get(url.trim()).unwrap();

    let mut file = File::create("image.png")?;

    _ = io::copy(&mut response, &mut file);
    println!("Image downloaded successfully!");
    Ok(())
}

// TODO: Simple Todo App
// Fn
// 1. Add Todo
// 2. List Todos
// 3. Complete Todo
// 4. Exit
//

use std::{
    io::Write,
    process::{self},
};

#[derive(Debug, Clone)]
struct Item {
    id: u32,
    title: String,
    completed: bool,
}

#[derive(Debug)]
struct Todo {
    items: Vec<Item>,
}

impl Todo {
    fn new() -> Todo {
        Todo { items: Vec::new() }
    }

    fn add_item(&mut self, title: String) {
        let item = Item {
            id: self.items.len() as u32 + 1,
            title,
            completed: false,
        };

        self.items.push(item.clone());

        println!("{:?}", item);
    }

    fn list_items(&self) {
        println!("List of Todos");
        for item in &self.items {
            let status = if item.completed { "[X]" } else { "[ ]" };
            println!("{} {} - {}", status, item.id, item.title);
        }
    }

    fn complete_item(&mut self, id: u32) {
        match self.items.iter_mut().find(|todo| todo.id == id) {
            Some(item) => {
                item.completed = true;
                println!("{} is completed", item.title);
            }
            None => print!(""),
        };
    }
}

fn main() {
    let mut todo = Todo::new();

    loop {
        println!("1. Add Item");
        println!("2. List Items");
        println!("3. Complete Item");
        println!("4. Exit");
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
                let mut title = String::new();
                println!("Enter the title of the new item:");
                std::io::stdin()
                    .read_line(&mut title)
                    .expect("Failed to read the line");
                todo.add_item(title.trim().to_string())
            }
            2 => todo.list_items(),
            3 => {
                let mut id = String::new();
                println!("Enter the ID of the completed item:");
                std::io::stdin()
                    .read_line(&mut id)
                    .expect("Failed to read the line");

                let id = match id.trim().parse::<u32>() {
                    Ok(id) => id,
                    Err(_) => {
                        println!("Failed to parse: Invalid data");
                        continue;
                    }
                };

                todo.complete_item(id)
            }
            4 => process::exit(0),
            _ => println!("Invalid option: Try again"),
        }
        println!("");
    }
}

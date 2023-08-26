mod lib;
use lib::TODO;
use std::io;

fn main() {
    let mut todo = TODO {
        list: Vec::new(),
        num_items: 0,
    };

    println!("Welcome! Enter Command:");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim();
        
        let args: Vec<&str> = input.split_whitespace().collect();
        
        if args.is_empty() {
            println!("Invalid command.");
            continue;
        }

        let command = args[0];

        if command == "exit" {
            break;

        } else if command == "add" && args.len() > 1 {
            let item = args[1..].join(" ");
            todo.add(item.to_string());

        } else if command == "done" && args.len() > 1 {
            for index in args[1..].iter() {
                if let Ok(parsed_index) = index.parse::<usize>() {
                    todo.done(parsed_index);
                } else {
                    println!("Failed to parse index: {}", index);
                }
            }

        } else if command == "list" {
            todo.list();
            println!();

        } else {
            println!("Invalid command or missing arguments.");
        }
    }
}

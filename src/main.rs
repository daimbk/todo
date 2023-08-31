use std::{ env, process };

mod lib;
use lib::TODO;

fn main() {
    // command logic
    let mut todo = TODO::new().expect("failed to initialize");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Atleast 2 arguments required. Use 'todo help'.");
        process::exit(1);
    }

    let command = &args[1];

    match command.as_str() {
        "add" => {
            let item = args[2..].join(" ");
            todo.add(item);
        }

        "done" => {
            for index in args[2..].iter() {
                if let Ok(parsed_index) = index.parse::<usize>() {
                    todo.done(parsed_index);

                } else {
                    println!("Failed to parse index: {}", index);
                    process::exit(1);
                }
            }
        }

        "remove" => {
            for index in args[2..].iter() {
                if let Ok(parsed_index) = index.parse::<usize>() {
                    let _ = todo.remove(parsed_index);

                } else {
                    println!("Failed to parse the index.");
                    process::exit(1);
                }
            }

            let _ = todo.remove_empty_lines();
        }

        "list" => {
            let _ = todo.list();
            println!();
        }

        _ => {
            println!("Unknown command. Use 'todo help'");
            process::exit(1);
        }
    }
}

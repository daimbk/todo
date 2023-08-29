mod lib;
use lib::TODO;
use std::env;
use std::process;

fn main() {
    // command logic
    let mut todo = TODO {
        list: Vec::new(),
        num_items: 0,
    };

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <command> [arguments]", args[0]);
        process::exit(1);
    }

    let command = &args[1];

    match command.as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Usage: {} add <item>", args[0]);
                process::exit(1);
            }
            let item = args[2..].join(" ");
            todo.add(item);
        }
        "done" => {
            if args.len() < 3 {
                println!("Usage: {} done <index>", args[0]);
                process::exit(1);
            }
            if let Ok(parsed_index) = args[2].parse::<usize>() {
                todo.done(parsed_index);
            } else {
                println!("Failed to parse the index.");
                process::exit(1);
            }
        }
        "remove" => {
            if args.len() < 3 {
                println!("Incorrect Command\nRun todo help for more information.");
                process::exit(1);
            }
            
            if let Ok(parsed_index) = args[2].parse::<usize>() {
                let _ = todo.remove(parsed_index);
            } else {
                println!("Failed to parse the index.");
                process::exit(1);
            }
        }
        "list" => {
            let _ = todo.list();
            println!();
        }

        "exit" => {
            println!("Exiting...");
            process::exit(0);
        }
        _ => {
            println!("Unknown command: {}", command);
            process::exit(1);
        }
    }
}

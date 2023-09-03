use std:: { env, process };
use std::fs::{ OpenOptions, File, remove_file };
use std::io::{ self, Write, BufRead, Result };

use whoami::username;

// strikethrough items when marked done
fn strikethrough_text(text: &mut String) {
    let strikethrough = "\x1B[9m";
    let reset_format = "\x1B[0m";

    *text = format!("{}{}{}", strikethrough, text, reset_format);
}

pub struct TODO {
    pub list: Vec<String>,
    pub file_path: String,
}

impl TODO {
    pub fn new() -> Result<Self> {
        let file_path = match env::consts::OS {
            // C:\Users\daim\Documents\TODO
            "windows" => format!("C:\\Users\\{}\\Documents\\TODO\\list.txt", username()),
            "linux" => format!("/home/{}/TODO/list.txt", username()),
            _ => {
                eprintln!("Unsupported operating system");
                process::exit(1);
            }
        };

        let todo = TODO { list: Vec::new(), file_path: file_path.to_string() };
        Ok(todo)
    }

    fn load_items(&mut self) -> Result<()> {
        // open file and load items
        let list_file = File::open(&self.file_path)?;
        let reader = io::BufReader::new(list_file);

        for line in reader.lines() {
            let line = line?;
            self.list.push(line);
        }

        Ok(())
    }

    // remove empty lines
    pub fn remove_empty_lines(&mut self) -> Result<()> {
        let list_file = File::open(&self.file_path)?;
        let reader = io::BufReader::new(list_file);

        for line in reader.lines() {
            let line = line?;
            self.list.push(line);
        }

        let mut file = File::create(&self.file_path)?;
        for item in &self.list {
            if item != "" {
                writeln!(file, "{}", item)?;
            }
        }

        self.list.clear();

        Ok(())
    }

    pub fn add(&mut self, mut item: String) {
        if item.is_empty() {
            eprintln!("todo add takes at least 1 argument");
            process::exit(1);
        }

        // store list
        let mut list_file = OpenOptions::new()
            .create(true) // create the file if it does not exist
            .append(true)
            .open(&self.file_path)
            .expect("Couldn't open the todofile");

        item = item + "\n";
        list_file
            .write(item.as_bytes())
            .expect("write file failed");
    }

    pub fn done(&mut self, mut item_index: usize) {
        let _ = self.load_items();

        item_index -= 1;

        if let Some(item) = self.list.get_mut(item_index) {
            strikethrough_text(item);

        } else {
            eprintln!("item {} does not exist", item_index);
            process::exit(1);
        }

        // write to file with strikethrough
        let mut file = File::create(&self.file_path).expect("file creation failed");
        for item in &self.list {
            writeln!(file, "{}", item).expect("file writing failed");
        }

        self.list.clear();
    }

    pub fn remove(&mut self, mut item_index: usize) -> io::Result<()> {
        self.load_items()?;

        item_index -= 1;

        if let Some(item) = self.list.get_mut(item_index) {
            *item = String::new();

        } else {
            eprintln!("item {} does not exist", item_index);
            process::exit(1);
        }

        let mut file = File::create(&self.file_path)?;
        for item in &self.list {
            writeln!(file, "{}", item)?;
        }
        
        // flush the list
        self.list.clear();

        Ok(())
    }

    pub fn reset(&self) {
        match remove_file(&self.file_path) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error while clearing todo file: {}", e)
            }
        };
    }

    pub fn list(&mut self) {
        println!();
        println!("TODO:");

        let _ = self.load_items();

        for (index, line) in self.list.iter().enumerate() {
            println!("{} {}", index + 1, line);
        }

        self.list.clear();
    }

    pub fn help(&self) {
        let help: &str = "\tUsage: todo 'command' 'arg'
        Example: todo add buy groceries
        
        Commands:
        - add [item]
        'adds the item to the todo list'

        - done [item number/s]
        'marks the item/s as completed'

        - remove [item number/s]
        'deletes the item/s from the list'

        - reset
        'deletes the item/s from the list'

        - list
        'displays the todo list'
        ";

        println!("{}", help);
    }
}

use std::process;
use std::fs::{ OpenOptions, File};
use std::io::{ self, Write, BufRead};

// strikethrough items when marked done
fn strikethrough_text(text: &mut String) {
    let strikethrough = "\x1B[9m";
    let reset_format = "\x1B[0m";

    *text = format!("{}{}{}", strikethrough, text, reset_format);
}

// struct for each item
pub struct TODO {
    pub list: Vec<String>,
    pub num_items: i32,
}

impl TODO {
    pub fn add(&mut self, mut item: String) {
        if item.is_empty() {
            eprintln!("todo add takes at least 1 argument");
            process::exit(1);
        }

        // store list
        // TODO: replace txt with database
        let mut list_file = OpenOptions::new()
            .create(true) // create the file if it does not exist
            .append(true)
            .open("list.txt")
            .expect("Couldn't open the todofile");

        item = item + "\n";
        list_file
            .write(item.as_bytes())
            .expect("write file failed");
    }

    pub fn done(&mut self, mut item_index: usize) {
        item_index -= 1;

        if let Some(item) = self.list.get_mut(item_index) {
            strikethrough_text(item);

        } else {
            eprintln!("item {} does not exist", item_index);
            process::exit(1);
        }
    }

    pub fn list(&self) -> io::Result<()> {
        println!();
        println!("TODO:");

        let list_file = File::open("list.txt")?;

        // create a BufReader to read the file line by line
        let reader = io::BufReader::new(list_file);

        for (index, line) in reader.lines().enumerate() {
            let line = line?;
            println!("{} {}", index + 1, line);
        }

        Ok(())
    }
}

use std::process;

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
    pub fn add(&mut self, item: String) {
        if item.is_empty() {
            eprintln!("todo add takes at least 1 argument");
            process::exit(1);
        }

        self.list.push(item);
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

    pub fn list(&self) {
        println!();
        println!("TODO:");
        for (index, item) in self.list.iter().enumerate() {
            println!("{} {}", index + 1, item);
        }
    }
}

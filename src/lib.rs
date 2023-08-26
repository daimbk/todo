use std::process;

// strikethrough items when marked done
fn strikethrough_text(text: &mut String) {
    let strikethrough = "\x1B[9m";
    let reset_format = "\x1B[0m";

    *text = format!("{}{}{}", strikethrough, text, reset_format);
}

pub struct TODO {
    pub item: String,
    pub done: bool,
}

impl TODO {
    pub fn add(&mut self, item: String) {
        if item.is_empty() {
            eprintln!("todo add takes at least 1 argument");
            process::exit(1);
        }

        self.item = item;
        self.done = false;
    }

    pub fn done(&mut self) {
        strikethrough_text(&mut self.item);
        self.done = true;
    }

    pub fn list_item(&self) {
        println!("{}", self.item);
    }
}

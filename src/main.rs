mod lib;
use lib::TODO;

fn main() {
    let mut todo = TODO {
        item: String::from("Buy groceries"),
        done: false,
    };

    todo.list_item();
    todo.done();
    todo.list_item();
}

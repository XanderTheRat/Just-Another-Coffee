use std::fs;

fn main() {
    let content = fs::read_to_string("coffee.txt");
    match content {
        Ok(f) => println!("Want a coffee ?\n{}", f),
        Err(_e) => println!("No more coffee."),
    };
}

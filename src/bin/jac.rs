use std::io;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut buffer = String::new();
    println!("Want a coffee ?");
    io::stdin().read_line(&mut buffer)?;
    if !(buffer.to_lowercase() == "n") {
        let content = fs::read_to_string("coffee");
        match content {
            Ok(f) => println!("\n{}", f),
            Err(e) => println!("No more coffee in cup : \n{}", e),
        };
    }
    else {
        println!("No more coffee.")
    }
    Ok(())

}

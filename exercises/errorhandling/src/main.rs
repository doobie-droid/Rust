use std::fs;

fn main() {
    let text  = fs::read_to_string("logs.txt");

    match text {
        Ok(t) => println!("{}", t),
        Err(e) => println!("Error: {}", e),
    }

    // println!("{:#?}", text);
}

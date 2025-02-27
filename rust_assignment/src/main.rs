use std::fs::File;
use std::io::{Read, BufReader, BufRead};

fn main() {
    let mut file = File::open("my_files/example3.txt").unwrap;
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("File contents:\n{}", contents);

    
    //writeln!(file, "Hello, Rust file operations!").unwrap();
    //writeln!(file, "This is a new line.").unwrap();
}
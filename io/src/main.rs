// use std::env;
use std::fs;
use std::io::prelude::*;

fn main() {
    // for (i, arg) in env::args().enumerate() {
    //     println!("argument {} is {}", i, arg);
    // }

    // let contents = fs::read_to_string("planets.txt").unwrap();
    // println!("File:\n{}", contents);
    // for line in contents.lines() {
    //     println!("line is '{}'", line);
    // }
    // let contents = fs::read("planets.txt").unwrap();
    // println!("File:\n{:?}", contents);

    let mut speech = String::new();
    speech.push_str("Hello");
    speech.push_str("You");
    speech.push_str("are");
    speech.push_str("little");
    speech.push_str("Rust");
    speech.push_str("Pusy");
    fs::write("./speech.txt", speech);

    let mut file = fs::OpenOptions::new().append(true).open("./planets.txt").unwrap();
    file.write(b"\nPluto");
}

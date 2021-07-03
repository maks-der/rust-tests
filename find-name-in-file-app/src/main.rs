use std::env::args;
use std::process::exit;
use std::fs::read_to_string;
use std::path::Path;

fn main() {
    let args = get_arguments();
    if find_name_in_file(&args.0, &args.1) {
        println!("\"{}\" exists in \"{}\" file", args.1, args.0);
    } else {
        println!("\"{}\" does NOT exist in \"{}\" file", args.1, args.0);
    }
}

fn get_arguments() -> (String, String) {
    if args().len() < 3 {
        exit_with_message("Please, provide file name and target name in file");
    }
    let vec: Vec<String> = args().collect(); 
    (vec[1].to_string(), vec[2].to_string())
}

fn find_name_in_file(path: &str, name: &str) -> bool {
    if !Path::new(&path).exists() {
        exit_with_message("File does not exist");
    }
    let file_string = read_to_string(path).unwrap();
    file_string.contains(&name)
}

fn exit_with_message(msg: &str) {
    println!("Exited with message: {}", msg);
    exit(0)
}
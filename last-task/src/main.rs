use std::process;
use std::fs;
use std::env;
use std::collections::HashMap;

fn main() {
    let mut file_path = String::new();
    for arg in env::args() {
        file_path = arg;      
    }

    print_line();
    let text: String = read_file(file_path);
    println!("Length of the text is {} symbols", text.len());
    
    let words_vector: Vec<String> = separate_words(text);
    println!("There are {} words in the text", words_vector.len());
    
    count_words(words_vector);
    print_line();
}


// Reading file:
// 1) Accept the path to a text via command-line argument
// 2) Example: > count_words.exe earth_to_the_moon.txt
// 3) Implement error handling as needed
fn read_file(path: String) -> String {
    let mut result = String::new();
    match fs::read_to_string(path) {
        Ok(s) => result = s,
        Err(e) => {
            println!("ERROR: {}", e);
            process::exit(1);
        },
    };
    return result;
}

// Separating words
// 1) Parse text into individuals words
// 2) Hint: use split_whitespace() method
// 3) Ignore capitalization
fn separate_words(text: String) -> Vec<String>{
    let mut result: Vec<String> = Vec::new();

    for word in text.split_whitespace() {
        result.push(String::from(word.to_lowercase()));
    }
    return result;
}

// Counting unique words
// 1) Keep track of how many times each unique words occured
// 2) Print most common words and how many times they occured
fn count_words(words_vector: Vec<String>) {
    let mut map: HashMap<String, i32> = HashMap::new();

    for word in words_vector {
        if map.contains_key(&word) {
            let count = map.get(&word).unwrap();
            map.insert(word, count + 1);
        } else {
            map.insert(word, 1);
        }
    }
    
    println!("{:?}", map);
    // 3) Keep in mind, there could be multiply "most common" words
}

fn print_line() {
    println!("___________________________");
}
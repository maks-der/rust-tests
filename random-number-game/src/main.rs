use rand::prelude::*;
use std::io;

fn main() {
    let num = thread_rng().gen_range(1..100);
    ask(num);
}


fn ask(num: i32) {
    println!("Gues the random number from 1 to 100");

    let mut repeat = true;
    let mut parsed_value: i32;
    while repeat {
        let mut user_num = String::new();
        io::stdin().read_line(&mut user_num);
        parsed_value = user_num.trim().parse().unwrap();
        repeat = !compare(parsed_value, num);
    }
}

fn compare(a: i32, b: i32) -> bool {
    let mut res = false;
    if a == b {
        res = true;
        println!("Correct!");
    } else if a < b {
        println!("Number is higher");
    } else {
        println!("Number is lower");
    }
    res
}

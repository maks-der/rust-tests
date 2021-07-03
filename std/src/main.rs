// use std::io;
// use rand;
use rand::prelude::*;


fn main() {
//    let mut buffer = String::new();
   
//    println!("Enter your buffer:");
//    io::stdin().read_line(&mut buffer);
//    println!("buffer is {}", buffer);

//    let number: i32 = buffer.trim().parse().unwrap();
//    println!("{} + 1 = {}", number, number + 1);

    // let num =  rand::random::<f64>();
    // println!("{}", num);

    let num = thread_rng().gen_range(1..11);
    println!("{}", num);

}
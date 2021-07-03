
fn main() {
    // say_hello();

    // let x = 1;
    // let y = 2;
    // say_the_sum(x, y);
    // say_number(x as i32);
    
    
    // println!("{:?}", square(13));


    // let cel_temp = 23.0;
    // let fahr_temp = celsius_to_fahrenhit(cel_temp);

    // assert_eq!(fahr_temp, 73.4);
    // println!("Test passed!");
    
    // let numbers: [i32;14] = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    
    // let min: i32 = get_min(numbers);
    // let max: i32 = get_max(numbers);
    // let mean: f64 = get_mean(numbers);

    // println!("\nmin: {}\nmax: {}\nmean: {}", min, max, mean);
    // assert_eq!(min, -18);
    // assert_eq!(max, 56);
    // assert_eq!(mean, 12.5);
    // println!("Test passed!");
}

// fn say_hello() {
//     println!("Hello!");
//     say_number(234);
// }

// fn say_number(number: i32) {
//     println!("number is {}", number);
// }

// fn say_the_sum(a: u8, b: u8) {
//     println!("sum is {}", a + b);
// }


// fn square(x: i32) -> (i32, i32) {
//     return (x, x * x);
// }


// fn celsius_to_fahrenhit(c: f64) -> f64 {
//     let f = (1.8 * c) + 32.0;
//     return f;
// }

// fn get_min(array: [i32;14]) -> i32 {
//     let mut res: i32 = array[0];
//     for num in array.iter() {
//         res = if res > *num {*num} else {res};
//     }
//     return res;
// }

// fn get_max(array: [i32;14]) -> i32 {
//     let mut res: i32 = array[0];
//     for num in array.iter() {
//         res = if res < *num {*num} else {res};
//     }
//     return res;
// }
// fn get_mean(array: [i32;14]) -> f64 {
//     let mut res: f64 = 0.;
//     for num in array.iter() {
//         res += *num as f64;
//     }
//     return res / array.len() as f64;
// }
fn main() {
    println!("\n-----------------------------\n");
    // let mut message = String::from("Earth!");
    // println!("{}", message);
    // message.push_str("Moon!");
    // println!("{}", message);

    // let mut rocket_fuel: String = String::from("RP-1");
    // let length = process_fuel(&mut rocket_fuel);

    // println!("rocket_fuel is {} and length is {}", rocket_fuel, length);



    let test1 = "We need more space.";
    assert_eq!(trim_spaces(test1), "We need more space.");
    
    let test2 = String::from("   There's space in front.");
    assert_eq!(trim_spaces(&test2), "There's space in front.");
    
    let test3 = String::from("There's space to the rear. ");
    assert_eq!(trim_spaces(&test3[..]), "There's space to the rear.");   
    
    let test4 = "  We're surrounded by space!    ";
    assert_eq!(trim_spaces(test4), "We're surrounded by space!");
    
    let test5 = "     ";
    assert_eq!(trim_spaces(test5), "");
    
    let test6 = "";
    assert_eq!(trim_spaces(test6), "");
    
    let test7 = " 🚀 ";
    assert_eq!(trim_spaces(test7), "🚀");
    println!("Tests passed!");
}

// fn process_fuel(propellant: &mut String) -> usize {
//     println!("processing propellant {}...", propellant);
//     propellant.push_str(" is highly flammable!");
//     let length = propellant.len();
//     length
// }

fn trim_spaces(string: &str) -> String {
    let mut result: String = String::from("");

    for byte in string.chars() {
        println!("{}", byte);
        if byte != ' ' {
            result.push(byte);
        }
    }
    return result;
}
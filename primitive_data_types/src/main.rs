fn main() {
    // let a = 10;
    // let b = 3.0;
    // let c = a as f64 / b;
    // println!("c is {0:010.3}\na is {1}\nc is {0}", c, a);

    // let mut value = 0b_0110_0100_u8;

    // println!("value is {0:08b} = {}", value);

    // value = !value;
    // println!("value is {0:08b} = {}", value);
    // value = value & 0b_0000_1111;
    // println!("value is {0:08b} = {}", value);
    // println!("bit 6 is {}", value & 0b_0010_0000);

    // value = value | 0b_1111_0100;
    // println!("value is {0:08b} = {}", value);
    // value = value ^ 0b_0001_1000;
    // println!("value is {0:08b} = {}", value);
    // value = value << 5;
    // println!("value is {0:08b} = {}", value);

    // value = value >> 5;
    // println!("value is {0:08b} = {}", value);

    // let a: bool = true;
    // let b: bool = false;

    // println!("a is {}, b is {}", a, b);
    // println!("NOT a is {}", !a);
    // println!("a AND b is {}", a & b);
    // println!("a OR b is {}", a | b);
    // println!("a XOR b is {}", a ^ b);

    // let letter = 'a';
    // let number = '1';
    // let finger = '\u{261D}';

    // println!("{}\n{}\n{}", letter, number, finger);

    // let a = 13;
    // let b = 2.3;
    // let c: f32 = 120.0;

    // let avarage = (a as f64 + b + c as f64) / 3.0;

    // assert_eq!(avarage, 45.1);
    // println!("Test passed!");

    // let mut letters: [char; 3] = ['a','b','c'];

    // letters[2] = 'x';
    // println!("{}", letters[0]);
    // println!("{}", letters[1]);
    // println!("{}", letters[2]);

    // let parking_lot = [
    //     [1, 2, 3],
    //     [4, 5, 6],
    //     [7, 8, 9]
    // ];

    // parking_lot

    let mut stuff: (u8, f32, char) = (10, 3.14, 'x');

    stuff.0 += 3;

    println!("{}", stuff.0);
    println!("{}", stuff.1);
    println!("{}", stuff.2);

    let (a, b, c) = stuff;
    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
}

#[derive(Debug)]

struct Rectangle<T, U> {
    width: T,
    height: U,
}

impl <T, U> Rectangle<T,U> {
    fn get_width(&self) -> &T {
        &self.width
    }
}

impl Rectangle<u8,u8> {
    fn get_perimetr(&self) -> u8 {
        2*&self.width + 2*&self.height
    }
}

fn get_biggest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        return a;
    }
    b
}


fn main() {
    let rec = Rectangle::<u8, u8> {
        width: 10,
        height: 30,
    };

    println!("{:?}", rec);    
    println!("width: {:?}", rec.get_width());    
    println!("perimetr: {:?}", rec.get_perimetr());
    println!("\n{}", get_biggest(1.0, 2.0));
}

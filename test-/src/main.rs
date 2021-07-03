#[derive(Debug)]

struct Animal {
    name: String,
    speed: f64,
}

impl Animal {
    fn accelerate(&mut self, speed: f64) {
        self.speed = speed;
    }

    fn new(name: &str) -> Animal {
        Animal {
            name: String::from(name),
            speed: 0.0
        }
    }
}

struct Color(u8,u8,u8);

fn main() {
    let mut anim = Animal::new("Dog");

    println!("{:?}", anim);
    anim.accelerate(12.3);
    println!("{:?}", anim);

    let black = Color(0,0,0);
    println!("#{}{}{}", black.0, black.1, black.2);
}

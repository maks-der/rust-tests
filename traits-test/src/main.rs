use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;


struct Satellite {
    name: String,
    velocity: f64
}

impl Display for Satellite {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result{
        write!(f, "\nSatellite:\t{}\nVelocity:\t{}\n", self.name, self.velocity)
    }
}


fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72,
    };
    println!("{}", hubble);
}

#[derive(Debug)]

struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}

impl Shuttle {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn add_fuel(&mut self, gallons: f64) {
        self.propellant += gallons;
    }

    fn new(name: &str) -> Shuttle {
        return Shuttle {
            name: String::from(name),
            crew_size: 0,
            propellant: 0.0,
        }
    }
}


struct Color(u8,u8,u8); // RGB
struct Point(u8,u8,u8); // XYZ

fn get_y(p: Point) -> u8 {
    p.1
}

fn main() {
/*     let mut vehicle = Shuttle {
        name: String::from("Aelita"),
        crew_size: 7,
        propellant: 234.324,
    };

    // let vehicle2 = Shuttle {
    //     name: String::from("Discovery"),
    //     ..vehicle
    // };

    let vehicle3 = Shuttle::new("Apollo");

    // vehicle.name = String::from("Titan");
    // vehicle.crew_size = 6;
    
    // println!("Name is {}", vehicle.name);
    // println!("vehicle is {:?}", vehicle);
    // println!("vehivle2 is {:?}", vehicle2);
    println!("vehivle3 is {:?}", vehicle3);

    let name = vehicle.get_name();
    println!("{}", name);

    println!("{}", vehicle.propellant);
    vehicle.add_fuel(1000.0);
    println!("{}", vehicle.propellant); */

    let red = Color(255,0,0);
    println!("{}", red.0);

    let coord = Point(0,1,0);
    let y = get_y(coord);
    println!("{}", y);
}

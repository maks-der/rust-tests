#[derive(Debug)]

enum Shape {
    Circle(f64),
    Rectangle(f64,f64),
    Triangle(f64,f64,f64),
}

fn main() {
    let my_shape = Shape::Rectangle(5.2, 4.3);

    println!("{:?}", my_shape);

    match my_shape {
        Shape::Circle(r) => println!("Circle with radius {}", r),
        Shape::Rectangle(x, y) => println!("{} x {} rectangle", x, y),
        Shape::Triangle(a,b,c) => println!("Triangle with sides {}, {}, {}", a, b, c),
    }
}

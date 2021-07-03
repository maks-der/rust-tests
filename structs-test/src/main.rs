struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new(w: f64, h: f64) -> Rectangle {
        Rectangle {
            width: w,
            height: h,
        }
    }

    fn get_area(&self) -> f64 {
        self.height * self.width
    }

    fn scale(&mut self, x: f64) {
        self.width = self.width * x;
        self.height = self.height * x;
    }
}

fn main() {
    let mut rect  = Rectangle::new(1.2, 3.4);
    assert_eq!(rect.get_area(), 4.08);
    rect.scale(0.5);
    assert_eq!(rect.get_area(), 1.02);
    println!("Test passed!");
}

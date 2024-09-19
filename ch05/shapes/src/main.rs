#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.height * self.width
    }

    fn length(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn new(w: f64, h: f64) -> Rectangle {
        Rectangle{width: w, height: h}
    }

    fn show(&self) {
        println!("Rectangle ({}, {}), S = {}, P = {}", self.width, self.height, self.area(), self.length());
    }
}
fn main() {
    let a = Rectangle::new(10.1, 2.3);
    a.show();
    println!("Short data: {:?}", a);
    println!("Prettified: {:#?}", a);
}

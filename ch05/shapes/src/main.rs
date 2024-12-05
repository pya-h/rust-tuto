#[derive(Debug)]
struct Point (f64, f64);

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
    top_left: Point
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.height * self.width
    }

    fn length(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn new_origin(w: f64, h: f64) -> Rectangle {
        Rectangle{width: w, height: h, top_left: Point (0.0, 0.0)}
    }

    fn new(top_left: Point, width: f64, height: f64) -> Rectangle {
        Rectangle{top_left, width, height}
    }

    fn show(&self) {
        println!("Rectangle ({}, {}), S = {}, P = {}", self.width, self.height, self.area(), self.length());
    }

    fn surrounds(&self, rect: &Rectangle) -> bool {
        self.top_left.0 <= rect.top_left.0 && rect.top_left.0 + rect.width <= self.top_left.0 + self.width
            && self.top_left.1 >= rect.top_left.1 && rect.top_left.1 - rect.height >= self.top_left.1 - self.height
    }
}

fn main() {
    let a = Rectangle::new_origin(10.1, 2.3);
    a.show();
    println!("Short data: {:?}", a);
    println!("Prettified: {:#?}", a);
    let x = Point(0.0 , 0.0);
    println!("{:?}", x);
    let b = Rectangle::new(Point(-1.0, 1.0), 12.0, 3.3);
    println!("a is inside b? {}", b.surrounds(&a));
}

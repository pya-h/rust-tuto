
#[derive(Debug)]
struct Point (f64, f64);

impl Point {
    fn value(&self) -> String {
        format!("({},{})", self.0, self.1)
    }
}

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

    fn new_zero(w: f64, h: f64) -> Rectangle {
        Rectangle{width: w, height: h, top_left: Point (0.0, 0.0)}
    }

    fn new(top_left: Point, width: f64, height: f64) -> Rectangle {
        Rectangle{top_left, width, height}
    }

    fn show(&self) {
        println!("Rectangle (({}, {}) @ {}), S = {}, P = {}", self.width, self.height, self.top_left.value(), self.area(), self.length());
    }

    fn surrounds(&self, rect: &Rectangle) -> bool {
        self.top_left.0 <= rect.top_left.0 && rect.top_left.0 + rect.width <= self.top_left.0 + self.width
            && self.top_left.1 >= rect.top_left.1 && rect.top_left.1 - rect.height >= self.top_left.1 - self.height
    }
}

struct Square {
    rect: Rectangle
}

impl Square {
    fn new(top_left: Point, length: f64) -> Self {
        Self {
            rect: Rectangle {top_left, width: length, height: length}
        }
    }

    fn new_zero(length: f64) -> Self {
        Self {
            rect: Rectangle {top_left: Point (0.0, 0.0), width: length, height: length}
        }
    }

    fn length(&self) -> f64 {
        self.rect.width * 4.0
    }

    fn surrounds(&self, other: &Square) -> bool {
        self.rect.surrounds(&other.rect)
    }

    fn surrounds_rect(&self, other: &Rectangle) -> bool {
        self.rect.surrounds(&other)
    }

    fn area(&self) -> f64 {
        self.rect.area()
    }

    fn show(&self) {
        println!("Square ({} @ {}), S = {}, P = {}", self.side(), self.top_left().value(), self.area(), self.length())
    }

    fn side(&self) -> f64 {
        self.rect.width
    }

    fn top_left(&self) -> &Point {
        &self.rect.top_left
    }
}

struct Circle {
    origin: Point,
    radius: f64
}

impl Circle {
    fn new_zero(radius: f64) -> Self {
        Self {origin: Point(0.0, 0.0), radius}
    }

    fn new(origin: Point, radius: f64) -> Self {
        Self {origin, radius}
    }

    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }

    fn length(&self) -> f64 {
        2.0 * self.radius * std::f64::consts::PI
    }

    fn show(&self) {
        println!("Circle ({} @ {}), S = {}, P = {}", self.radius, self.origin.value(), self.area(), self.length())
    }
}

enum Shape {
    Rectangle(Rectangle),
    Square(Square),
    Circle(Circle)
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Self::Rectangle(rect) => rect.area(),
            Self::Square(sq) => sq.area(),
            Self::Circle(circle) => circle.area()
        }
    }

    fn length(&self) -> f64 {
        match self {
            Self::Rectangle(rect) => rect.length(),
            Self::Square(sq) => sq.length(),
            Self::Circle(circle) => circle.length()
        }
    }

    fn show(&self) {
        match self {
            Self::Rectangle(rect) => rect.show(),
            Self::Square(sq) => sq.show(),
            Self::Circle(circle) => circle.show()
        };
    }
}

fn main() {
    let a = Rectangle::new_zero(10.1, 2.3);
    a.show();
    println!("Short data: {:?}", a);
    println!("Prettified: {:#?}", a);
    let x = Point(0.0 , 0.0);
    println!("{:?}", x);
    let b = Rectangle::new(Point(-1.0, 1.0), 12.0, 3.3);
    println!("a is inside b? {}", b.surrounds(&a));

    let sq = Square::new_zero(1.0);
    sq.show();

    let circle = Shape::Circle(Circle::new_zero(5.0));

    circle.show();
    
    let square = Shape::Square(Square::new(Point(2.0, 1.0), 2.0));
    println!("square: S = {}, P = {}", square.area(), square.length())
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    login_count: u64,
    is_active:  bool

}

// struct Test {
//     x: &str
// }
fn new_user(username: String, email: String)  -> User {
    User {username, // init shorthand syntax
        email,
        login_count: 1,
        is_active: true
    }
}

fn struct_update_syntax(username: String, email: String, old_user: &User) -> User {
    // println!("{}", old_user.email);
    User {
        username,
        email,
        ..*old_user
    }
}
fn struct_update_syntax_bymove(email: String, old_user: User) -> User {
    User {
        email,
        ..old_user
    }
}

#[derive(Debug)]
//  structs need to impl Debug to be printed by println! MACRO, directly
struct Rectangle {
    width: f64,
    height: f64
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }

    fn square(size: f64) -> Rectangle {
        Rectangle { width:size, height: size }
    }

    fn print(&self) {
        println!("{} {{width: {}, height: {}}}", if self.width == self.height {"Square"} else {"Rectangle"},
            self.width, self.height);
    }
}
fn area(rectangle: &Rectangle) -> f64 {
    rectangle.width * rectangle.height
}

fn main() {
    let user1 = User{
        username: String::from("khatmr"),
        email: String::from("fucker@gmail.com"),
        login_count: 2,
        is_active: false
    };
    println!("user1: {:?}", user1);
    let user2 = new_user(String::from("fuck"), String::from("yeah@no.com"));
    let user3 = struct_update_syntax(String::from("paya"), String::from("paya@py@no.com"), &user2);
    // println!("{}", user2.username);
    println!("user2: {:?}", user2);
    let user4 = struct_update_syntax_bymove(String::from("paya@py"), user1);
// user1 moved
    // how to send padam without borrowing, using &?
    // println!("{} {} {} {}", user3.username, user3.email,
    //     user3.login_count, user3.is_active);
    println!("user3: {:#?}", user3);
    // println!("{} {} {} {}", user4.username, user4.email,
    // user4.login_count, user4.is_active);
    println!("user4: {:#?} is the last one", user4);

    // tuple structs:
    // struct Name(type1, type2, ...)
    struct Color(i32, i32, i32);
    struct Point(f32, f32, f32);
    struct Vector(f32, f32, f32);
    // Point and Vector, although they have the same types and same number of params, are different
    // you cant pass Point inbstead of Vector argument(and reverse) to a function
    let v = Vector(3.0, 4.0, 4.5);
    let r1 = Rectangle{width: 10.4, height: 5.5};
    let r2: Rectangle = Rectangle { width:2.1, height: 3.2 };
    // Debug formatted struct
    println!("area of {:?} is {}, r1 can hold r2? {}", r1, r1.area(),
        if r1.can_hold(&r2) { "yep" } else { "nope" }); // or &r1.Area()
    // println!("area of {:#?} is {}", r1, area(&r1));
    // let test = Test{x: "hey me"} // error
    let sq = Rectangle::square(5.5);
    sq.print();

}
// user2 moved

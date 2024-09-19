struct User {
    name: String,
    email: String,
    age: u8,
    is_male: bool
}

fn new_user(name: String, email: String, age: u8, is_male: bool) -> User {
    User {name, email, age, is_male}
}

fn opposite_sex(user: User) -> User {
    User {
        is_male: !user.is_male,
        ..user
    }
}

impl User {
    fn opposite_sex(self) -> User {
        User {
            is_male: !self.is_male,
            ..self
        }
    }

    fn new(name: String, email: String, age: u8, is_male: bool) -> User {
        User {name, email, age, is_male}
    }

    fn show(&self) {
        println!("{{\n\tname: {}\n\temail: {}\n\tage: {}\n\tgender: {}\n}}\n", self.name, self.email, self.age,
             if self.is_male {"Male"} else {"Female"})
    }
}
fn main() {
    // different types of instantiate:
    let u1 = User {
        name: String::from("user1"),
        email: String::from("us1@mail.com"),
        age: 20,
        is_male: true
    };
    let u2 = new_user( String::from("user2"), String::from("us2@mail.com"), 25, false);

    u1.show();
    u2.show();
    let u3 = opposite_sex(u1);
    u3.show();
    // u1.show(); // this throws, since u1 is moved
    let u4 = u2.opposite_sex();
    u4.show();
    // u2.show(); // u2 is moved too
    let u2 = User::new(String::from("user2 plus"), String::from("us2plus@mail.com"), 25, false);
    u2.show(); // now that u2 is re-assigned it can be used again

    struct Color (i8, i8, i8); // tuple struct
    impl Color {
        fn show(&self) {
            println!("({}, {}, {})", self.0, self.1, self.2);
        }
    }
    let color: Color = Color (2, 4, 10);
    color.show();

}


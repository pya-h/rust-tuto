enum Gender {
    Male,
    Female
}

struct Person {
    gender: Gender,
    name: String
}

impl Person {
    fn introduce(&self) -> String {
        let gender = match self.gender {
            Gender::Male => "Male", 
            Gender::Female => "Female"
        };
        
        self.name.clone() + &String::from(" is a ") + gender // String + &str
    }
}

// prp kind of enum
#[derive(Debug)]
enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(String)
}

fn main() {
    let p1: Person = Person{name: String::from("pya"), gender: Gender::Male};
    println!("{}", p1.introduce());
    let ip1 = IpAddress::V4(192, 168, 0, 1);
    let ip2 = IpAddress::V6(String::from("::1"));
    // println!("ip1 = {}, ip2 = {}", ip1)
}

use std::fmt::Display;

struct Person<'a> {
    name: &'a str,
}

impl<'a> Person<'a> {
    fn say_hello(&self, lastname: &str) -> &'static str {
        println!("Hello, {} {}", self.name, lastname);
        "lastname" // Notice: Lifetime elision rule gives sets return value lifetime as 'a which is wrong here; so this will throw an error.
    }
    // Correct form lifetime generic:
    fn say_hello_correct<'b>(&self, lastname: &'b str) -> &'b str {
        println!("Hello, {} {}", self.name, lastname);
        lastname
    }

}

fn lifetime_with_generi_def<'x, 'y, T, U>(s1: &'x str, s2: &'x str, p1: &'y T, p2: &'y U) -> &'x str 
    where T: Display, U: Display 
{
    println!("P1: {} P2: {}", p1, p2);
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
} 
fn main() {
    let lastname = "doe";
    let mut l2 = "doe2";
    {
        let person = Person { name: "john" };
        l2 = person.say_hello(lastname);
    }
    println!("{}", l2); // This throws lifetime error

    let fname = "first";
    let lname = "last";
    {
        let a = 10;
        let b = "param2";

        let longest = lifetime_with_generi_def(fname, lname, &a, &b);        
        println!("longest is: {}, with the lifetime equal or less than fname and lname", longest);
    }
}

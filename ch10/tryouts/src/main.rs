struct Person<'a> {
    name: &'a str,
}

impl<'a> Person<'a> {
    fn say_hello(&self, lastname: &str) -> &str {
        println!("Hello, {} {}", self.name, lastname);
        lastname // Notice: Lifetime elision rule gives sets return value lifetime as 'a which is wrong here; so this will throw an error.
    }
    // Correct form lifetime generic:
    fn say_hello_correct<'b>(&self, lastname: &'b str) -> &'b str {
        println!("Hello, {} {}", self.name, lastname);
        lastname
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
}

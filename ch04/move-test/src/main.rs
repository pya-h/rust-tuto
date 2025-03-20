fn main() {
    let s1 = String::from("Test");
    let s2 = s1;
    println!("s2 = {}", s2);
    // println!("s1 = {}", s1); // => Error: s1 moved to s2

    let mut t1 = String::from("Another test");
    let t2 = t1;
    t1 = String::from("Next"); // t1 was moved but, here assigned with the address of another string so cvan be used again.

    println!("t2 = {}", t2);
    println!("t1 = {}", t1);
}

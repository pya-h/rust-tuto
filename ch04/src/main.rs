
fn first_word(s: &str) -> (&str , usize) {
    let mut i: usize = 0;
    let bytes = s.as_bytes();
    while i < bytes.len() {
        if bytes[i] == b' ' {
            return (&s[0..i], i);
        }
        i += 1;
    }
    (&s[0..s.len()], s.len())
}

fn second_word(s: &str) -> (&str, usize){
    let (_, index) = first_word(s);
    if index >= s.len() - 1 {
        return ("", 0);
    }
    let s = &s[index+1..];

    for (i, &ch) in s.as_bytes().iter().enumerate() {
        if ch == b' ' {
            return (&s[..i], index + i)
        }
    }
    (&s[..], s.len() + index)
}
fn main() {
    let s: (&str, usize) = second_word("Hello,world fuck u!");
    println!("{} {}", s.0, s.1);
}

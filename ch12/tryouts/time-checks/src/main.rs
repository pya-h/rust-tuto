use std::time::{SystemTime, UNIX_EPOCH};

fn get_time() -> u64 {
    let now = SystemTime::now();
    now.duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

fn main() {
    println!("Hello, world! {}", get_time());
}

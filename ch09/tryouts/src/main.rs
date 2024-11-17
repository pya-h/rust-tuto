use std::{fs::File, io::ErrorKind};

fn main() {
    let f: File = match File::open(path) {
        Ok(f) => f,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            // whatever
        },
        Err(error) => {
            // general error handler
        }
    }
}

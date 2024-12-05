use std::{fs::File, io, io::ErrorKind};
use std::io::Read;

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut f: File = match File::open(path) {
        Ok(f) => f,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            return match File::create(path) {
                Ok(_f) => {
                    read_file(path)
                },
                Err(e) => { Err(e) }
            };
        },
        Err(error) => {
            return Err(error);
        }
    };
    let mut contents = String::new();
    match f.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(e) => Err(e),
    }
}

fn read_file_propagation_shortcut(path: &str) -> Result<String, io::Error> {
    let mut f: File = File::open(path)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(contents)
}

fn read_file_short(path: &str) -> Result<String, io::Error> {
    let mut contents = String::new();
    File::open(path)?.read_to_string(&mut contents)?;
    Ok(contents)
}

fn panicking_read_file(path: &str) -> String {
    let mut f: File = File::open(path).expect("Can not open the file, check if file exists!");

    let mut contents = String::new();

    f.read_to_string(&mut contents).unwrap();

    contents
}

fn main() {
    println!("Choose Read Method:\n\t1. Propagating Error handling\n\t2. Direct Panic Handling");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let choice: u8 = input.trim().parse().expect("Invalid choice!");
    match choice {
        1 => {
            match read_file("text.txt") {
                Err(e) => println!("Failed to read file: {}", e),
                Ok(s) => println!("File content: {}", s),
            }
        }
        2 => {
            println!("File content: {}", panicking_read_file("text.txt"))
        }
        _ => println!("Invalid choice!"),
    };
    println!("App finished with no panics!");
}
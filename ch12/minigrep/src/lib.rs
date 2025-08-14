pub mod config;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub fn run(cfg: config::Config) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(cfg.filename())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("File successfully read:\n\t{}\n", contents);
    Ok(())
}
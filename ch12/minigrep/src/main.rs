mod config;

use std::process;

use config::Config;

fn main() {
    let cmd_args: Vec<String> = std::env::args().collect();
    let config = Config::new(&cmd_args).unwrap_or_else(|err: String| {
        println!("minigrep failed: {}", err);
        process::exit(1);
    });
    println!("{}", config.to_string());
}

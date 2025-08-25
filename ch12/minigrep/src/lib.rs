pub mod config;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub fn run(cfg: config::Config) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(cfg.filename())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("File successfully read:\n\t{}\n", contents);
    let (l, v) = search(&cfg.query(), &contents, cfg.options());
    println!("{:?}, {:?}", l, v);
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str, options: &config::SearchOptions) -> (Vec<usize>, Vec<&'a str>) {
    let mut results: Vec<&str> = Vec::new();
    let mut lines: Vec<usize> = Vec::new();
    if contents.trim().len() == 0 {
        panic!("No contents to search!");
    }
    if query.trim().len() == 0 {
        panic!("Empty query string provided!")
    }

    let query_lower = if options.case_sensitive { String::new() } else { query.to_lowercase() };

    for (line_number, line) in contents.lines().enumerate() {
        let matched = match options.case_sensitive {
            true => line.contains(query),
            false => line.to_lowercase().contains(&query_lower)
        };
        if matched {
            results.push(line);
            lines.push(line_number);
        }
    }
    if lines.len() == 0 {
        panic!("No match found!");
    }
    (lines, results)
    // Notice: The reason that returning lines is not causing trouble (one might say lines is defined in the function,
    // and will be freed at the end), is that on the last line lines is moved, back to the calling function...
}

#[cfg(test)]
mod test {
    use crate::config::SearchOptions;

    use super::*;

    #[test]
    fn search_specific_line() {
        let query = "fast";
        let contents = "\
        Rust:
            safe, fast, productive.
            Pick three.";
        let (line_numbers, lines) = search(query, contents, &SearchOptions::defaults());
        // using different kinds of asserts
        assert_eq!(line_numbers.len(), 1);
        assert!(line_numbers == [1]);
        assert_ne!(lines.len(), 0);
        assert_eq!(lines[0].trim(), "safe, fast, productive.");
        assert!(lines.len() == 1)
    }

    #[test]
    #[should_panic(expected = "No match found!")]
    fn search_not_found() {
        search("X",  "ABCDEFG",&SearchOptions::defaults());
    }

    #[test]
    #[should_panic(expected = "No contents to search!")]
    fn search_no_contents() {
        search("S", "",&SearchOptions::defaults());
    }

    #[test]
    #[should_panic(expected = "Empty query string")]
    fn search_empty_query() {
        search("", "Text",&SearchOptions::defaults());
    }
}
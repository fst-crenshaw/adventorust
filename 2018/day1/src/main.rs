use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // Create a path to the desired file
    let path = Path::new("hello.txt");
    let display = path.display();
    let mut result = 0;

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => (),
    }

    for s in s.trim_end().split("\n") {
        let eos = &s[1..];
        let val = eos.parse::<i32>().unwrap();

        if s.starts_with("+") {
            result = result + val;
        } else if s.starts_with("-") {
            result = result - val;
        }
    }

    println!("{}", result);
}

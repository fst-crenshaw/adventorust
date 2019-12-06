use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;

fn main() {
    // Create a path to the desired file
    let path = Path::new("input.txt");

    // printable-version of the path
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => print!("Success Opening File!\n"), //{} contains:\n{}", display, s),
    }
    // `file` goes out of scope, and the "input.txt" file gets closed

    // Puzzle!
    let mut twos = 0;
    let mut threes = 0;

    for s in s.split("\n") {
        // end of file
        if s.len() == 0 {
            break;
        }
        // begin solving here
        let mut letters = HashMap::new();

        for c in s.chars() {
            let val = letters.entry(c).or_insert(0);
            *val += 1;
            /* let val = letters.get_mut(&c);
            let val_from_hash = match val {
                Some(num) => *num,
                _ => 0
            };
            letters.insert(c, val_from_hash + 1);
            */
        }
        // let any_twos = letters.values().any(2);

        for (_, val) in letters.iter() {
           // println!("key: {} val: {}", key, val);
            if *val == 2 {
                twos += 1;
                break;
            }
        }
        for (_, val) in letters.iter() {
           // println!("key: {} val: {}", key, val);
            if *val == 3 {
                threes += 1;
                break;
            }
        }
        //println!("{:?}", letters);
    }
    let result = twos * threes;
    println!("{}", result);
}

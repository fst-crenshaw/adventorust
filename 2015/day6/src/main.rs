use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod elevator;
mod elves;
mod lights;

fn main() {
    crate::elevator::elevator::test_elevator();
    crate::elves::elves::test_elves();
    crate::lights::lights::test_lights();

    // Create a path to the desired file
    let path = Path::new("lights.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();

    let mut decorations = crate::lights::lights::LightGrid {
        grid: [[0; 1000]; 1000],
    };

    //    decorations.PrettyPrint();

    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => println!("File read"),
    }

    println!("");

    for line in s.split('\n') {
        //println!("{}", line);
        let i = crate::lights::lights::parse(line);
        //println!("{:?}", i);
        crate::lights::lights::execute(&i, &mut decorations);
    }

    decorations.PrettyPrint();

    println!("");

    println!("Number of Lights On: {}", decorations.NumberOn());
}

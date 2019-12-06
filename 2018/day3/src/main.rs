use regex::Regex;
use std::fs;

#[derive(Default)] // If we derive the Default Trait here...declaration is easier.
pub struct Fabric {
    pub inches: [[u32; 10]; 10], // This could be a vector!  :)
}

impl Fabric {
    pub fn pretty_print(&self) {
        for row in self.inches.iter() {
            for inch in row.iter() {
                print!("{}", inch);
            }
            println!("");
        }
    }
}

    // There are two crates called anyhow and thiserror.  If you were to put
    // a
    let re = Regex::new(r".?@ (\d{1,3}),(\d{1,3}): (\d{1,3})x(\d{1,3})")?;

    for claim in s.split("\n") {
        // Note that a claim looks like:
        // #1 @ 179,662: 16x27
        let cap = re.captures(claim).unwrap();
        let x: usize = cap[1].parse().unwrap();
        let y: usize = cap[2].parse().unwrap();
        let width: usize = cap[3].parse().unwrap();
        let height: usize = cap[4].parse().unwrap();

        // println!("x: {:?}, y: {:?}, width: {:?}, height {:?}", &cap[1], &cap[2], &cap[3], &cap[4]);
        // println!("x: {:?}, y: {:?}, width: {:?}, height {:?}", x, y, width, height);

        for i in x..x + width {
            for j in y..y + height {
                fabric.inches[i][j] += 1;
            }
        }
    }

fabric.pretty_print();

// Rust offers the ability to derive Traits.  Let's say we'd like to
// compare one Fabric to another Fabric
//#[derive(PartialOrd)]

fn main() -> std::io::Result<()> {
    // Change main to return a result.

    // Is this the most concise way to read a file into a string?
    // There might be another function that you could call that's lines() that
    // might prevent you from reading it all in at once; instead you could
    // read it line-by-line.  This may not matter for this puzzle.
    let s = fs::read_to_string("claims.txt")?;

    // If claims.txt is not found, the program prints out:
    //  Error: Os { code: 2, kind: NotFound, message: "No such file or directory" }
    // If we wanted to do something more elegant with the error, we would
    // use a match statement.  But the ? is stopping the program and printing
    // the error for us.

    let s = s.trim();
    let mut fabric: Fabric = Default::default();

    // There aren't constructors in Rust.  There's no `new` keyword.
    // By convention, you just do a function called new, and usually there would
    // be parameters like lxw.
    // It's how I listen.

    fabric.pretty_print();

    // The person who wrote the Regex crate is called "BurntSushi".
    // There's another useful crate called `ripgrep` that is like super fast grep.
    // That's a crate that's fun to learn about.
    // There's another one called `fd` that's a find alternative.
    // These are great places to read existing Rust code and learn idioms in Rust
    // since the concepts are approachable and the code is well-written.

    // It's better to get out of main quickly.
    // Call something like "dothestuff" function
    


    // This wraps nothing inside of an Ok.  To specify a nothing object,
    // one uses ().  The thing that is okay is nothing, which isn't
    // really returning anything.
    Ok(())
}

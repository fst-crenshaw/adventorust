use regex::Regex;
use std::fs;

pub struct Fabric {
    pub inches: [[u32; 10]; 10],
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

fn main() {
    let s = match fs::read_to_string("claims.txt") {
        Ok(s) => s,
        Err(_) => panic!("Ahhh!"),
    };
    let s = s.trim();

    let mut fabric = Fabric {
        inches: [[0; 10]; 10],
    };

    fabric.pretty_print();

    let re = Regex::new(r".?@ (\d{1,3}),(\d{1,3}): (\d{1,3})x(\d{1,3})").unwrap();

    for claim in s.split("\n") {
        // claim looks like:
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
}

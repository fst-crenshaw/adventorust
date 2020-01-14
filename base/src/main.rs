use std::fs;

fn main() {
    let s = fs::read_to_string("input.txt").unwrap();
    let s = s.trim();

    for line in s.split('\n') {
        println!("{}", line);
    }
}

use std::fs;
mod elevator;

fn main() {
    crate::elevator::elevator::test_elevator();

    let s = match fs::read_to_string("input.txt") {
        Ok(s) => s,
        Err(_) => panic!("Ahhh!"),
    };
    let s = s.trim();

    let result = crate::elevator::elevator::floor(s);

    println!("\nSanta ought to go to floor {}.", result);
}

use anyhow::Result;
use std::fs;

mod elves;

fn main() -> Result<()> {
    crate::elves::elves::test_elves();

    let s = fs::read_to_string("input.txt")?;
    let s = s.trim();

    let mut total: u32 = 0;

    println!("");

    for line in s.split('\n') {
        if crate::elves::elves::is_nice(line) == true {
            total = total + 1;
        }
    }

    println!("");
    println!("Number of nice strings: {}", total);
    Ok(())
}

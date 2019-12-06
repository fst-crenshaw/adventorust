use anyhow::{anyhow, Error, Result};
use regex::Regex;
use std::fs;

struct Present(u32, u32, u32);

impl Present {
    fn from_str(s: &str) -> Result<Present, Error> {
        let re = Regex::new(r"(\d{1,2})x(\d{1,2})x(\d{1,3})")?;
        let cap = re
            .captures(s)
            .ok_or(anyhow!("invalid Present dimensions"))?;

        return Ok(Present(cap[1].parse()?, cap[2].parse()?, cap[3].parse()?));
    }
}

/// Given a Christmas Present's dimensions, determine how
/// many square feet of wrapping paper are necessary for wrapping.
///    https://adventofcode.com/2015/day/2
fn get_paper(p: &Present) -> u32 {
    let Present(l, w, h) = p;

    let dims = [l * w, w * h, h * l];

    let min_dim = dims.iter().fold(std::u32::MAX, |mut min_val, d| {
        min_val = std::cmp::min(min_val, *d);
        min_val
    });

    (2 * l * w + 2 * w * h + 2 * h * l) + min_dim
}

// *** Tests
fn test_get_paper(p: &Present, expected: u32) -> bool {
    if get_paper(p) == expected {
        return true;
    }
    false
}

fn main() -> Result<()> {
    assert!(test_get_paper(&Present(2, 3, 4), 58));
    assert!(test_get_paper(&Present(1, 1, 10), 43));

    let s = match fs::read_to_string("input.txt") {
        Ok(s) => s,
        Err(_) => panic!("Ahhh!"),
    };

    let mut total: u32 = 0;
    let s = s.trim();

    for line in s.split('\n') {
        let p = Present::from_str(line)?;
        total = total + get_paper(&p);
    }

    println!("");

    println!("Total square feet of paper necessary: {}.", total);

    Ok(())
}

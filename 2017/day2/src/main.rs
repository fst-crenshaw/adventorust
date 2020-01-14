use std::fs;

fn process_row(row: &str) -> u64 {
    let mut min: u64 = std::u64::MAX;
    let mut max: u64 = std::u64::MIN;

    if row.is_empty() {
        // An early return prefers the phrase "return".
        return 0; // I think return must be here.
    }

    for term in row.split_whitespace() {
        let term = term.parse::<u64>().unwrap();
        if term < min {
            min = term;
        }
        if term > max {
            max = term;
        }
    }

    let diff = max - min;

    println!("{}", diff);

    // At the end of the function, it's considered idiomatic
    // to not use the "return" phrase.
    // diff;  With the semi-colon, there's no return value.  Compiler error happens.

    diff
}

fn main() {
    let s = fs::read_to_string("input.txt").unwrap();
    let s = s.trim();

    let sum = s.split('\n').map(process_row).sum::<u64>();

    /*
    for term in s.split('\n').map(process_row) {
        sum = sum + term;
    }
     */

    println!("\n\n{}", sum);

    /*
     for line in s.split('\n') {
       let diff = process_row(line);
       sum = sum + diff;
     }
    */
}

#[cfg(test)]
mod tests {
    use crate::process_row;

    #[test]
    fn test_one() {
        let input = "5 10 15 20";
        assert_eq!(process_row(input), 15);
    }
}

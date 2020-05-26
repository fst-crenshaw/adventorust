/*
You arrive at the Venus fuel depot only to discover it's protected by
a password. The Elves had written the password on a sticky note, but
someone threw it out.

However, they do remember a few key facts about the password:

- It is a six-digit number.
- The value is within the range given in your puzzle input.
- Two adjacent digits are the same (like 22 in 122345).
- Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).

- Other than the range rule, the following are true:
--- 111111 meets these criteria (double 11, never decreases).
--- 223450 does not meet these criteria (decreasing pair of digits 50).
--- 123789 does not meet these criteria (no double).

How many different passwords within the range given in your puzzle input meet these criteria?

Your puzzle input is 357253-892942.
*/

// 012334  -- True
// 012345  -- False
fn has_adj_digits(input: i32) -> bool {

    let input_as_string = input.to_string();

    let mut prev = 0 as char;
    
    for i in input_as_string.chars() {
	if i == prev {
	    return true;
	}		
	prev = i;
    }
    false
}
    
fn main() {
    let start = 357253 as i32;
    // let end = 892942 as i32;
    let end = 357293 as i32;
    
    let mut yo_results = Vec::new();

    for i in start..end {
	if has_adj_digits(i) == true {
	    yo_results.push(i);
	}
    }

    dbg!(has_adj_digits(12345));
    dbg!(has_adj_digits(11122));

    dbg!("{}", yo_results);
}

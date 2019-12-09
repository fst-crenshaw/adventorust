use regex::Regex;
use std::collections::HashMap;
use std::fs;

/// An Exp, or expression, takes one of the following forms:
///   u32 -> <id>
///   NOT <id> -> <id>
enum Exp<'a> {
    Literal(&'a u32),
    UnaryExp(fn(a: u32) -> u32, &'a str),
}

/// An Assignment is a variable and an expression.
struct Assignment<'a> {
    var: &'a str,
    exp: Exp<'a>,
}

fn aoc_not(a: u32) -> u32 {
    return !a;
}

fn parse<'a>(s: &'a str) -> Result<Box<Assignment<'a>>, ()> {
    // Parse the assignment `NOT <id> -> <id>`
    if s.contains("NOT") {
        let re = Regex::new(r"^NOT (?P<exp>\w{1,2}) -> (?P<var>\w{1,2})$").unwrap();
        let cap = re.captures(&s).unwrap();
        let exp = Exp::UnaryExp(aoc_not, cap.name("exp").unwrap().as_str());

        let assign = Assignment {
            var: cap.name("var").unwrap().as_str(),
            exp: exp,
        };
        return Ok(Box::new(assign));
    }
    Err(())
}

fn main() {
    let mut expressions = HashMap::new();
    let s = fs::read_to_string("input_sample.txt").unwrap();
    let s = s.trim();

    for line in s.split('\n') {
        let assignment = parse(line).unwrap();
        expressions.insert(assignment.var, assignment.exp);
    }
}

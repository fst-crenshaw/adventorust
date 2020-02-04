use regex::Regex;
use std::collections::HashMap;
use std::fs;

/// An Exp, or expression, takes one of the following forms:
///   u32 -> <id>
///   NOT <id> -> <id>
#[derive(Debug, PartialEq)]
enum Exp<'a> {
    Literal(u32),
    UnaryExp(fn(a: u32) -> u32, &'a str),
    BinaryExp(fn(a: u32, b: u32) -> u32, &'a str, &'a str),
}

/// An Assignment is a variable and an expression.
#[derive(Debug, PartialEq)]
struct Assignment<'a> {
    var: &'a str,
    exp: Exp<'a>,
}

struct BlockSet<'a> {
    var: &'a str,
    blocked_by: &'a str,
}

fn aoc_and(a: u32, b: u32) -> u32 {
    return a & b;
}

fn aoc_or(a: u32, b: u32) -> u32 {
    return a | b;
}

fn aoc_not(a: u32) -> u32 {
    return !a;
}

fn parse<'a>(s: &'a str) -> Result<Box<Assignment<'a>>, ()> {
    let exp;
    let cap;

    // Parse the assignment `<id> AND <id> -> <id>`
    if s.contains("AND") {
        let re =
            Regex::new(r"^(?P<exp1>\w{1,2}) AND (?P<exp2>\w{1,2}) -> (?P<var>\w{1,2})$").unwrap();
        cap = re.captures(&s).unwrap();
        exp = Exp::BinaryExp(
            aoc_and,
            cap.name("exp1").unwrap().as_str(),
            cap.name("exp2").unwrap().as_str(),
        );
    }
    // Parse the assignment `<id> OR <id> -> <id>`
    else if s.contains("OR") {
        let re =
            Regex::new(r"^(?P<exp1>\w{1,2}) OR (?P<exp2>\w{1,2}) -> (?P<var>\w{1,2})$").unwrap();
        cap = re.captures(&s).unwrap();
        exp = Exp::BinaryExp(
            aoc_or,
            cap.name("exp1").unwrap().as_str(),
            cap.name("exp2").unwrap().as_str(),
        );
    }
    // Parse the assignment `NOT <id> -> <id>`
    else if s.contains("NOT") {
        let re = Regex::new(r"^NOT (?P<exp>\w{1,2}) -> (?P<var>\w{1,2})$").unwrap();
        cap = re.captures(&s).unwrap();
        exp = Exp::UnaryExp(aoc_not, cap.name("exp").unwrap().as_str());
    // Parse the assignment `u32 -> <id>`
    } else {
        let re = Regex::new(r"^(?P<literal>\d{1,4}) -> (?P<var>\w{1,2})$").unwrap();
        cap = re.captures(&s).unwrap();
        exp = Exp::Literal(cap.name("literal").unwrap().as_str().parse().unwrap());
    }
    let assign = Assignment {
        var: &cap.name("var").unwrap().as_str(),
        exp,
    };
    return Ok(Box::new(assign));
}

/// Given an expression, attempt to evaluate it.  If it cannot be
/// evaluated, return None.
fn eval(exp: &Exp) -> Option<u32> {
    match exp {
        Exp::Literal(val) => Some(*val),
        /*
                Exp::UnaryExp(f, exp) => match exp.parse() {
                    Ok(val) => Some(f(val)),
                    _ => None,
                },
        */
        _ => None,
    }
}

fn main() {
    let mut expressions = HashMap::new();
    //let mut values = HashMap::new();

    let s = fs::read_to_string("input_sample.txt").unwrap();
    let s = s.trim();

    for line in s.split('\n') {
        let assignment = parse(line).unwrap();
        println!("{:?}", assignment);
        expressions.insert(assignment.var, assignment.exp);
    }
}

#[cfg(test)]
mod tests {
    use crate::{aoc_and, aoc_or, parse, Assignment, Exp};

    #[test]
    fn parse_simple_assignment() {
        assert_eq!(
            parse("123 -> x").unwrap(),
            Box::new(Assignment {
                var: "x",
                exp: Exp::Literal(123)
            })
        );
    }

    #[test]
    fn parse_assignment_with_and() {
        assert_eq!(
            parse("x AND y -> d").unwrap(),
            Box::new(Assignment {
                var: "d",
                exp: Exp::BinaryExp(aoc_and, "x", "y")
            })
        );
    }

    #[test]
    fn parse_assignment_with_or() {
        assert_eq!(
            parse("x OR y -> e").unwrap(),
            Box::new(Assignment {
                var: "e",
                exp: Exp::BinaryExp(aoc_or, "x", "y")
            })
        );
    }
}

/*
123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i
*/

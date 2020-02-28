use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, PartialEq)]
enum Term<'a> {
    Literal(u32),
    Variable(&'a str),
}

/// An expression is any of
///  123
///  1 AND y
///  x AND y
///  0 OR x
///  x OR 0
///  NOT y
///  NOT 1
#[derive(Debug, PartialEq)]
enum Exp<'a> {
    Literal(u32),
    UnaryExp(fn(a: u32) -> u32, Term<'a>),
    BinaryExp(fn(a: u32, b: u32) -> u32, Term<'a>, Term<'a>),
}

/// An Assignment is an identifier and an expression.
///    i.e., <exp> -> <id>
///
#[derive(Debug, PartialEq)]
struct Assignment<'a> {
    id: &'a str,
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

/// Given a string representing a term in an expression
/// return its type, either a Variable (like "x") or
/// a Literal (like 1).
fn reduce<'a>(s: &'a str) -> Term<'a> {
    let term = s.clone();

    // Is it a number?
    let maybe_number = term.parse::<u32>();

    match maybe_number {
        Ok(number) => return Term::Literal(number),
        Err(_) => return Term::Variable(term),
    }
}

fn parse<'a>(s: &'a str) -> Result<Box<Assignment<'a>>, ()> {
    let exp;
    let cap;

    // Parse the assignment `<id> AND <id> -> <id>`
    if s.contains("AND") {
        let re =
            Regex::new(r"^(?P<exp1>\w{1,2}) AND (?P<exp2>\w{1,2}) -> (?P<id>\w{1,2})$").unwrap();
        cap = re.captures(&s).unwrap();
        exp = Exp::BinaryExp(
            aoc_and,
            reduce(cap.name("exp1").unwrap().as_str()),
            reduce(cap.name("exp2").unwrap().as_str()),
        );
    }
    // Parse the assignment `<id> OR <id> -> <id>`
    else if s.contains("OR") {
        let re =
            Regex::new(r"^(?P<exp1>\w{1,2}) OR (?P<exp2>\w{1,2}) -> (?P<id>\w{1,2})$").unwrap();
        cap = re.captures(&s).unwrap();
        exp = Exp::BinaryExp(
            aoc_or,
            reduce(cap.name("exp1").unwrap().as_str()),
            reduce(cap.name("exp2").unwrap().as_str()),
        );
    }
    // Parse the assignment `NOT <id> -> <id>`
    else if s.contains("NOT") {
        let re = Regex::new(r"^NOT (?P<exp>\w{1,2}) -> (?P<id>\w{1,2})$").unwrap();
        cap = re.captures(&s).unwrap();
        exp = Exp::UnaryExp(aoc_not, reduce(cap.name("exp").unwrap().as_str()));
    // Parse the assignment `u32 -> <id>`
    } else {
        let re = Regex::new(r"^(?P<literal>\d{1,4}) -> (?P<id>\w{1,2})$").unwrap();
        cap = re.captures(&s).unwrap();
        exp = Exp::Literal(cap.name("literal").unwrap().as_str().parse().unwrap());
    }
    let assign = Assignment {
        id: &cap.name("id").unwrap().as_str(),
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
        expressions.insert(assignment.id, assignment.exp);
    }
}

#[cfg(test)]
mod tests {
    use crate::{aoc_and, aoc_not, aoc_or, parse, Assignment, Exp, Term};

    #[test]
    fn parse_simple_assignment() {
        assert_eq!(
            parse("123 -> x").unwrap(),
            Box::new(Assignment {
                id: "x",
                exp: Exp::Literal(123)
            })
        );
    }

    #[test]
    fn parse_assignment_with_and() {
        assert_eq!(
            parse("x AND y -> d").unwrap(),
            Box::new(Assignment {
                id: "d",
                exp: Exp::BinaryExp(aoc_and, Term::Variable("x"), Term::Variable("y"))
            })
        );
        assert_eq!(
            parse("1 AND y -> d").unwrap(),
            Box::new(Assignment {
                id: "d",
                exp: Exp::BinaryExp(aoc_and, Term::Literal(1), Term::Variable("y"))
            })
        );

        assert_eq!(
            parse("x AND 0 -> d").unwrap(),
            Box::new(Assignment {
                id: "d",
                exp: Exp::BinaryExp(aoc_and, Term::Variable("x"), Term::Literal(0))
            })
        );
    }

    #[test]
    fn parse_assignment_with_or() {
        assert_eq!(
            parse("x OR y -> e").unwrap(),
            Box::new(Assignment {
                id: "e",
                exp: Exp::BinaryExp(aoc_or, Term::Variable("x"), Term::Variable("y"))
            })
        );
        assert_eq!(
            parse("0 OR y -> e").unwrap(),
            Box::new(Assignment {
                id: "e",
                exp: Exp::BinaryExp(aoc_or, Term::Literal(0), Term::Variable("y"))
            })
        );
        assert_eq!(
            parse("y OR 0 -> e").unwrap(),
            Box::new(Assignment {
                id: "e",
                exp: Exp::BinaryExp(aoc_or, Term::Variable("y"), Term::Literal(0))
            })
        );
    }

    #[test]
    fn parse_assignment_with_not() {
        assert_eq!(
            parse("NOT y -> e").unwrap(),
            Box::new(Assignment {
                id: "e",
                exp: Exp::UnaryExp(aoc_not, Term::Variable("y"))
            })
        );
        assert_eq!(
            parse("NOT 1 -> e").unwrap(),
            Box::new(Assignment {
                id: "e",
                exp: Exp::UnaryExp(aoc_not, Term::Literal(1))
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

use regex::Regex;
use std::collections::HashMap;
use std::fs;
use thiserror::Error;

#[derive(Debug, Error)]
enum Error {
    #[error("Cannot parse {0}")]
    ParseError(String),
}

/// An OpKind describes the kinds of operations available in the language
#[derive(Debug)]
enum OpKind {
    Binary,
    Unary,
    Shift,
}

/// A BinaryOp, or an operation accepting two expressions as operands,
/// represents the collection of binary operations available in the
/// language.
#[derive(Debug)]
enum BinaryOp {
    AND,
    OR,
}

/// A ShiftOp, or shift operation, represents the collection
/// of shift operations available in the language.
#[derive(Debug)]
enum ShiftOp {
    LSHIFT,
    RSHIFT,
}

/// An Exp, or expression, takes one of the following forms:
///   u32 -> <id>               Literal
///   NOT <id> -> <id>          UnaryExp
///   <id> AND <id> -> <id>     BinaryExp
///   <id> OR <id> -> <id>      BinaryExp
///   <id> LSHIFT u32 -> <id>   ShiftExp
///   <id> RSHIFT u32 -> <id>   ShiftExp
#[derive(Debug, PartialEq)]
enum Exp<'a> {
    Literal(u32),
    UnaryExp(fn(a: u32) -> u32, &'a str),
    BinaryExp(fn(a: u32, b: u32) -> u32, &'a str, &'a str),
    ShiftExp(fn(a: u32, b: u32) -> u32, &'a str, u32),
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

fn aoc_lshift(a: u32, b: u32) -> u32 {
    return a << b;
}

fn aoc_rshift(a: u32, b: u32) -> u32 {
    return a >> b;
}

fn aoc_not(a: u32) -> u32 {
    return !a;
}

fn construct_assignment<'a>(
    o: OpKind,
    re: &'a str,
    f: fn(u32, u32) -> u32,
    s: &'a str,
) -> Result<Box<Assignment<'a>>, Error> {
    let re = Regex::new(re).unwrap();
    let cap = re.captures(&s);
    match cap {
        None => Err(Error::ParseError(s.to_owned())),
        v => {
            let v = v.unwrap();
            let var = &v.name("var").unwrap().as_str();
            let exp1 = v.name("exp1").unwrap().as_str();

            let exp;

            match o {
                OpKind::Binary => {
                    exp = Exp::BinaryExp(f, exp1, v.name("exp2").unwrap().as_str());
                }
                OpKind::Shift => {
                    exp = Exp::ShiftExp(
                        f,
                        exp1,
                        v.name("exp2").unwrap().as_str().parse::<u32>().unwrap(),
                    );
                }
                _ => panic!(),
            }

            let assign = Assignment { var, exp };
            Ok(Box::new(assign))
        }
    }
}

fn parse_binary<'a>(op: BinaryOp, s: &'a str) -> Result<Box<Assignment<'a>>, Error> {
    let and_regexp = r"^(?P<exp1>\w{1,2}) AND (?P<exp2>\w{1,2}) -> (?P<var>\w{1,2})$";
    let or_regexp = r"^(?P<exp1>\w{1,2}) OR (?P<exp2>\w{1,2}) -> (?P<var>\w{1,2})$";

    match op {
        BinaryOp::AND => construct_assignment(OpKind::Binary, and_regexp, aoc_and, s),
        BinaryOp::OR => construct_assignment(OpKind::Binary, or_regexp, aoc_or, s),
    }
}

fn parse_shift<'a>(op: ShiftOp, s: &'a str) -> Result<Box<Assignment<'a>>, Error> {
    let lshift_regexp = r"^(?P<exp1>\w{1,2}) LSHIFT (?P<exp2>\d{1,2}) -> (?P<var>\w{1,2})$";
    let rshift_regexp = r"^(?P<exp1>\w{1,2}) RSHIFT (?P<exp2>\d{1,2}) -> (?P<var>\w{1,2})$";

    match op {
        ShiftOp::LSHIFT => construct_assignment(OpKind::Shift, lshift_regexp, aoc_lshift, s),
        ShiftOp::RSHIFT => construct_assignment(OpKind::Shift, rshift_regexp, aoc_rshift, s),
    }
}

fn parse<'a>(s: &'a str) -> Result<Box<Assignment<'a>>, Error> {
    let exp;
    let cap;

    if s.contains("AND") {
        return parse_binary(BinaryOp::AND, s);
    } else if s.contains("OR") {
        return parse_binary(BinaryOp::OR, s);
    } else if s.contains("LSHIFT") {
        return parse_shift(ShiftOp::LSHIFT, s);
    } else if s.contains("RSHIFT") {
        return parse_shift(ShiftOp::RSHIFT, s);
    } else if s.contains("NOT") {
        let re = Regex::new(r"^NOT (?P<exp>\w{1,2}) -> (?P<var>\w{1,2})$").unwrap();
        let captures = re.captures(&s);
        if let None = captures {
            return Err(Error::ParseError(s.to_owned()));
        }
        cap = captures.unwrap();
        exp = Exp::UnaryExp(aoc_not, cap.name("exp").unwrap().as_str());
    } else {
        let re = Regex::new(r"^(?P<literal>\d{1,4}) -> (?P<var>\w{1,2})$").unwrap();
        let captures = re.captures(&s);
        if let None = captures {
            return Err(Error::ParseError(s.to_owned()));
        }
        cap = captures.unwrap();
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
    use crate::{aoc_and, aoc_lshift, aoc_not, aoc_or, aoc_rshift, eval, parse};
    use crate::{Assignment, Exp};

    #[test]
    fn eval_literal() {
        assert_eq!(eval(&Exp::Literal(23)), Some(23));
    }

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

        assert_eq!(
            parse("xx AND yy -> dd").unwrap(),
            Box::new(Assignment {
                var: "dd",
                exp: Exp::BinaryExp(aoc_and, "xx", "yy")
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

    #[test]
    fn parse_assignment_with_lshift() {
        assert_eq!(
            parse("h LSHIFT 33 -> k").unwrap(),
            Box::new(Assignment {
                var: "k",
                exp: Exp::ShiftExp(aoc_lshift, "h", 33)
            })
        );
    }

    #[test]
    fn parse_assignment_with_rshift() {
        assert_eq!(
            parse("x RSHIFT 2 -> e").unwrap(),
            Box::new(Assignment {
                var: "e",
                exp: Exp::ShiftExp(aoc_rshift, "x", 2)
            })
        );
    }

    #[test]
    fn parse_assignment_with_not() {
        assert_eq!(
            parse("NOT x -> h").unwrap(),
            Box::new(Assignment {
                var: "h",
                exp: Exp::UnaryExp(aoc_not, "x")
            })
        );
    }
    #[test]
    fn parsing_bad_expressions_must_fail_gracefully() {
        // Make a bunch of calls that should not panic.
        assert!(parse("AND").is_err());
        assert!(parse("OR").is_err());
        assert!(parse("NOT").is_err());
        assert!(parse("AND y -> xx").is_err());
        assert!(parse("a NOT y -> h").is_err());
        assert!(parse("y y -> yy").is_err());
        assert!(parse("->").is_err());
        assert!(parse("22").is_err());
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

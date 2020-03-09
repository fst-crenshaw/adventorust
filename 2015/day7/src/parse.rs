use crate::types::{Assignment, Exp, Term};

use regex::Regex;

fn aoc_and(a: u16, b: u16) -> u16 {
    return a & b;
}

fn aoc_or(a: u16, b: u16) -> u16 {
    return a | b;
}

fn aoc_not(a: u16) -> u16 {
    return !a;
}

fn aoc_lshift(a: u16, b: u16) -> u16 {
    let shift = b.checked_shl(a.into());
    match shift {
        Some(v) => v,
        None => 0,
    }
}

fn aoc_rshift(a: u16, b: u16) -> u16 {
    let shift = b.checked_shr(a.into());
    match shift {
        Some(v) => v,
        None => 0,
    }
}

/// Given a string representing a term in an expression return its
/// type, either a Variable (like "x") or a Literal (like 1).
fn reduce<'a>(s: &'a str) -> Term {
    let term = s.to_string();

    // Is it a number?
    let maybe_number = term.parse::<u16>();

    match maybe_number {
        Ok(number) => return Term::Literal(number),
        Err(_) => return Term::Variable(term.clone()),
    }
}

fn reduce_lhs<'a>(s: &'a str) -> Exp {
    let term = s.to_string();

    // Is it a number?
    let maybe_number = term.parse::<u16>();

    match maybe_number {
        Ok(number) => return Exp::Literal(number),
        Err(_) => return Exp::Variable(term.clone()),
    }
}

/// Given a string representing an assignment return its parsed
/// Assignment structure.
pub fn parse<'a>(s: &'a str) -> Result<Box<Assignment>, ()> {
    let exp;
    let cap;

    // Parse any assignment containing a binary expression.
    if s.contains("RSHIFT") || s.contains("LSHIFT") || s.contains("AND") || s.contains("OR") {
        let re = Regex::new(
            r"^(?P<exp1>\w{1,2}) (?P<op>RSHIFT|LSHIFT|AND|OR) (?P<exp2>\w{1,2}) -> (?P<id>\w{1,2})$",
        )
        .unwrap();
        cap = re.captures(&s).unwrap();

        let f = match cap.name("op").unwrap().as_str() {
            "RSHIFT" => aoc_rshift,
            "LSHIFT" => aoc_lshift,
            "OR" => aoc_or,
            "AND" => aoc_and,
            _ => panic!("You can't always get what you want."),
        };

        exp = Exp::BinaryExp(
            f,
            reduce(cap.name("exp1").unwrap().as_str()),
            reduce(cap.name("exp2").unwrap().as_str()),
        );
    }
    // Parse the assignment `NOT <id> -> <id>`
    else if s.contains("NOT") {
        let re = Regex::new(r"^NOT (?P<exp>\w{1,2}) -> (?P<id>\w{1,2})$").unwrap();
        cap = re.captures(&s).unwrap();
        exp = Exp::UnaryExp(aoc_not, reduce(cap.name("exp").unwrap().as_str()));
    // Parse the assignment `u16 -> <id>` or `<id> -> <id>`
    } else {
        let re = Regex::new(r"^(?P<lhs>\w{1,5}) -> (?P<id>\w{1,2})$").unwrap();
        cap = re.captures(&s).unwrap();
        exp = reduce_lhs(cap.name("lhs").unwrap().as_str());
    }
    let assign = Assignment {
        id: cap.name("id").unwrap().as_str().to_string(),
        exp,
        val: None,
    };
    return Ok(Box::new(assign));
}

#[cfg(test)]
mod tests {
    use super::{aoc_and, aoc_not, aoc_or, parse};
    use crate::{Assignment, Exp, Term};
    use std::cmp::Ordering;

    #[test]
    fn compare_expressions() {
        let mut a1;
        let mut a2;

        a1 = parse("1 -> b").unwrap();
        a2 = parse("1 -> b").unwrap();
        assert_eq!(Ordering::Equal, a1.cmp(&a2));

        a2 = parse("1 -> c").unwrap();
        assert_eq!(Ordering::Less, a1.cmp(&a2));

        a2 = parse("NOT 1 -> d").unwrap();
        assert_eq!(Ordering::Less, a1.cmp(&a2));

        a2 = parse("NOT x -> a").unwrap();
        assert_eq!(Ordering::Less, a1.cmp(&a2));

        a2 = parse("y AND x -> a").unwrap();
        assert_eq!(Ordering::Less, a1.cmp(&a2));

        a1 = parse("1 AND x -> c").unwrap();
        a2 = parse("y AND x -> a").unwrap();
        assert_eq!(Ordering::Less, a1.cmp(&a2));
    }
    #[test]
    fn parse_simple_assignment() {
        assert_eq!(
            parse("123 -> x").unwrap(),
            Box::new(Assignment {
                id: "x".to_owned(),
                exp: Exp::Literal(123),
                val: None
            })
        );
    }

    #[test]
    fn parse_assignment_with_and() {
        assert_eq!(
            parse("x AND y -> d").unwrap(),
            Box::new(Assignment {
                id: "d".to_owned(),
                exp: Exp::BinaryExp(
                    aoc_and,
                    Term::Variable("x".to_owned()),
                    Term::Variable("y".to_owned())
                ),
                val: None
            })
        );
        assert_eq!(
            parse("1 AND y -> d").unwrap(),
            Box::new(Assignment {
                id: "d".to_owned(),
                exp: Exp::BinaryExp(aoc_and, Term::Literal(1), Term::Variable("y".to_owned())),
                val: None
            })
        );

        assert_eq!(
            parse("x AND 0 -> d").unwrap(),
            Box::new(Assignment {
                id: "d".to_owned(),
                exp: Exp::BinaryExp(aoc_and, Term::Variable("x".to_owned()), Term::Literal(0)),
                val: None
            })
        );
    }

    #[test]
    fn parse_assignment_with_or() {
        assert_eq!(
            parse("x OR y -> e").unwrap(),
            Box::new(Assignment {
                id: "e".to_owned(),
                exp: Exp::BinaryExp(
                    aoc_or,
                    Term::Variable("x".to_owned()),
                    Term::Variable("y".to_owned())
                ),
                val: None
            })
        );
        assert_eq!(
            parse("0 OR y -> e").unwrap(),
            Box::new(Assignment {
                id: "e".to_owned(),
                exp: Exp::BinaryExp(aoc_or, Term::Literal(0), Term::Variable("y".to_owned())),
                val: None
            })
        );
        assert_eq!(
            parse("y OR 0 -> e").unwrap(),
            Box::new(Assignment {
                id: "e".to_owned(),
                exp: Exp::BinaryExp(aoc_or, Term::Variable("y".to_owned()), Term::Literal(0)),
                val: None
            }),
        );
    }

    #[test]
    fn parse_assignment_with_not() {
        assert_eq!(
            parse("NOT y -> e").unwrap(),
            Box::new(Assignment {
                id: "e".to_owned(),
                exp: Exp::UnaryExp(aoc_not, Term::Variable("y".to_owned())),
                val: None
            })
        );
        assert_eq!(
            parse("NOT 1 -> e").unwrap(),
            Box::new(Assignment {
                id: "e".to_owned(),
                exp: Exp::UnaryExp(aoc_not, Term::Literal(1)),
                val: None
            })
        );
    }
}

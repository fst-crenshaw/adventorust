use regex::Regex;
use std::collections::HashMap;
use std::fs;

/// The program state is the set of variables whose values are known
/// and the set of variables whose assigned expressions cannot yet be
/// evaluated.
#[derive(Debug)]
struct State {
    known: HashMap<String, u16>,
    free: HashMap<String, Exp>,
}

impl State {
    fn new() -> Self {
        Self {
            known: HashMap::new(),
            free: HashMap::new(),
        }
    }
}

/// A term in an expression is one of a variable (like "x") or an
/// unsigned integer (like 1).
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Term {
    Literal(u16),
    Variable(String),
}

/// An expression is any of:
///  123
///  1 AND y
///  x AND y
///  0 OR x
///  x OR 0
///  NOT y
///  NOT 1
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Exp {
    Literal(u16),
    Variable(String),
    UnaryExp(fn(a: u16) -> u16, Term),
    BinaryExp(fn(a: u16, b: u16) -> u16, Term, Term),
}

/// An Assignment is an identifier and an expression.
///    i.e., <exp> -> <id>
///
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Assignment {
    exp: Exp,
    id: String,
}

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
    println!("{} << {} = {:?}", b, a, shift);
    match shift {
        Some(v) => v,
        None => 0,
    }
}

fn aoc_rshift(a: u16, b: u16) -> u16 {
    let shift = b.checked_shr(a.into());
    println!("{} << {} = {:?}", b, a, shift);
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
fn parse<'a>(s: &'a str) -> Result<Box<Assignment>, ()> {
    let exp;
    let cap;

    println!("Parsing {}", s);

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
        println!("Parsing {}", s);
        let re = Regex::new(r"^(?P<lhs>\w{1,5}) -> (?P<id>\w{1,2})$").unwrap();
        cap = re.captures(&s).unwrap();
        exp = reduce_lhs(cap.name("lhs").unwrap().as_str());
    }
    let assign = Assignment {
        id: cap.name("id").unwrap().as_str().to_string(),
        exp,
    };
    return Ok(Box::new(assign));
}

fn eval<'a>(assign: &'a Assignment, state: &'a mut State) -> bool {
    // Attempt to evaluate the expression.  If expression evaluation
    // returns None, then we add the expression to the set of free
    // variables.  If expression evaluation returns Some(_) then we
    // add it to known program state.
    let maybe_evaluated_expr: Option<u16> = eval_expr(&assign.exp, &state.known);

    // The expression has only Literals and may be immediately
    // evaluated.
    if let Some(e) = maybe_evaluated_expr {
        println!("Adding to known values: {} = {}.", assign.id, e);
        state.known.insert(assign.id.to_owned(), e);
        return true;
    }

    let my_exp = assign.exp.clone();
    state.free.insert(assign.id.to_owned(), my_exp);
    return false;
}

fn eval_expr(exp: &Exp, known: &HashMap<String, u16>) -> Option<u16> {
    match exp {
        Exp::Literal(el) => Some(*el),
        Exp::UnaryExp(f, Term::Literal(el)) => Some(f(*el)),
        Exp::BinaryExp(f, Term::Literal(el1), Term::Literal(el2)) => Some(f(*el1, *el2)),
        Exp::Variable(v) => {
            let known_val = known.get(v);
            if let Some(kv) = known_val {
                return Some(*kv);
            } else {
                return None;
            }
        }
        Exp::UnaryExp(f, Term::Variable(v)) => {
            let known_val = known.get(v);
            if let Some(kv) = known_val {
                return Some(f(*kv));
            } else {
                return None;
            }
        }
        Exp::BinaryExp(f, Term::Literal(el), Term::Variable(v)) => {
            let known_val = known.get(v);
            if let Some(kv) = known_val {
                return Some(f(*el, *kv));
            } else {
                return None;
            }
        }
        Exp::BinaryExp(f, Term::Variable(v), Term::Literal(el)) => {
            let known_val = known.get(v);
            if let Some(kv) = known_val {
                return Some(f(*el, *kv));
            } else {
                return None;
            }
        }
        Exp::BinaryExp(f, Term::Variable(v1), Term::Variable(v2)) => {
            let known_val1 = known.get(v1);
            let known_val2 = known.get(v2);
            let pair = (known_val1, known_val2);

            match pair {
                (Some(kv1), Some(kv2)) => Some(f(*kv1, *kv2)),
                _ => None,
            }
        }
    }
}

fn main() {
    let mut assignments = Vec::new();
    let mut state = State::new();

    let s = fs::read_to_string("input.txt").unwrap();
    let s = s.trim();

    // Gather and parse all the assignments in the input.
    for line in s.split('\n') {
        let assignment = parse(line).unwrap();
        assignments.push(assignment);
    }

    // Sort the assignments
    assignments.sort();

    for _ in 0..5 {
        for a in assignments.iter() {
            println!("Evaluating: {:?}", a);
            eval(a, &mut state);
        }
    }

    println!("The value of a: {:?}", state.known.get("a"));
    println!("The value of lx: {:?}", state.known.get("lx"));
}

#[cfg(test)]
mod tests {
    use crate::{
        aoc_and, aoc_not, aoc_or, eval, eval_expr, parse, Assignment, Exp, HashMap, State, Term,
    };
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
    fn eval_assignments() {
        let mut state = State::new();
        let mut my_assign;

        my_assign = parse("1 -> x").unwrap();
        eval(&my_assign, &mut state);

        my_assign = parse("1 AND 1 -> z").unwrap();
        eval(&my_assign, &mut state);

        my_assign = parse("NOT z -> r").unwrap();
        eval(&my_assign, &mut state);

        my_assign = parse("1 AND r -> s").unwrap();
        eval(&my_assign, &mut state);

        my_assign = parse("z OR r -> t").unwrap();
        eval(&my_assign, &mut state);

        assert_eq!(state.known.get("x"), Some(1).as_ref());
        assert_eq!(state.known.get("z"), Some(1).as_ref());
        assert_eq!(state.known.get("r"), Some(0).as_ref());
        assert_eq!(state.known.get("s"), Some(0).as_ref());
        assert_eq!(state.known.get("t"), Some(1).as_ref());
    }

    #[test]
    fn eval_expressions() {
        let mut my_assign;
        let known = HashMap::new();

        my_assign = parse("1 AND 0 -> d").unwrap();
        assert_eq!(eval_expr(&my_assign.exp, &known), Some(0));

        my_assign = parse("1 AND 1 -> d").unwrap();
        assert_eq!(eval_expr(&my_assign.exp, &known), Some(1));

        my_assign = parse("0 OR 1 -> d").unwrap();
        assert_eq!(eval_expr(&my_assign.exp, &known), Some(1));

        my_assign = parse("1 OR 1 -> d").unwrap();
        assert_eq!(eval_expr(&my_assign.exp, &known), Some(1));

        my_assign = parse("0 OR 0 -> d").unwrap();
        assert_eq!(eval_expr(&my_assign.exp, &known), Some(0));
    }

    #[test]
    fn parse_simple_assignment() {
        assert_eq!(
            parse("123 -> x").unwrap(),
            Box::new(Assignment {
                id: "x".to_owned(),
                exp: Exp::Literal(123)
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
                )
            })
        );
        assert_eq!(
            parse("1 AND y -> d").unwrap(),
            Box::new(Assignment {
                id: "d".to_owned(),
                exp: Exp::BinaryExp(aoc_and, Term::Literal(1), Term::Variable("y".to_owned()))
            })
        );

        assert_eq!(
            parse("x AND 0 -> d").unwrap(),
            Box::new(Assignment {
                id: "d".to_owned(),
                exp: Exp::BinaryExp(aoc_and, Term::Variable("x".to_owned()), Term::Literal(0))
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
                )
            })
        );
        assert_eq!(
            parse("0 OR y -> e").unwrap(),
            Box::new(Assignment {
                id: "e".to_owned(),
                exp: Exp::BinaryExp(aoc_or, Term::Literal(0), Term::Variable("y".to_owned()))
            })
        );
        assert_eq!(
            parse("y OR 0 -> e").unwrap(),
            Box::new(Assignment {
                id: "e".to_owned(),
                exp: Exp::BinaryExp(aoc_or, Term::Variable("y".to_owned()), Term::Literal(0))
            })
        );
    }

    #[test]
    fn parse_assignment_with_not() {
        assert_eq!(
            parse("NOT y -> e").unwrap(),
            Box::new(Assignment {
                id: "e".to_owned(),
                exp: Exp::UnaryExp(aoc_not, Term::Variable("y".to_owned()))
            })
        );
        assert_eq!(
            parse("NOT 1 -> e").unwrap(),
            Box::new(Assignment {
                id: "e".to_owned(),
                exp: Exp::UnaryExp(aoc_not, Term::Literal(1))
            })
        );
    }
}

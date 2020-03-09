mod parse;
mod types;

use self::parse::parse;
use self::types::{Assignment, Exp, State, Term};

use std::collections::HashMap;
use std::fs;

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
    use crate::parse::parse;
    use crate::types::State;
    use crate::{eval, eval_expr, HashMap};

    #[test]
    fn eval_assignments() {
        let mut state = State::new();
        let mut my_assign;

        my_assign = parse("1 -> x").unwrap();
        eval(&my_assign, &mut state);

        my_assign = parse("1 AND 1 -> z").unwrap();
        eval(&my_assign, &mut state);

        // Bitwise NOT of an unsigned 16 1 is 15 0's
        my_assign = parse("NOT z -> r").unwrap();
        eval(&my_assign, &mut state);

        my_assign = parse("1 AND r -> s").unwrap();
        eval(&my_assign, &mut state);

        my_assign = parse("z OR r -> t").unwrap();
        eval(&my_assign, &mut state);

        assert_eq!(state.known.get("x"), Some(1).as_ref());
        assert_eq!(state.known.get("z"), Some(1).as_ref());
        assert_eq!(state.known.get("r"), Some(65534).as_ref());
        assert_eq!(state.known.get("s"), Some(0).as_ref());
        assert_eq!(state.known.get("t"), Some(65535).as_ref());
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
}

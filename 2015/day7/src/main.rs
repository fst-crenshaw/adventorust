mod parse;
mod types;

use self::parse::parse;
use self::types::{Assignment, Exp, State, Term};

use std::collections::HashMap;
use std::fs;

// Return true if a new valuation was created.
fn eval<'a>(assign: &'a mut Assignment, state: &'a mut State) -> bool {
    // An expression have have been previously evaluated.  If so, its
    // valuation is available in the Assignment's val field.
    if let Some(_) = assign.val {
        return false; // assign.val;
    }

    // Attempt to evaluate the expression.  If expression evaluation
    // returns None, then we add the expression to the set of free
    // variables.  If expression evaluation returns Some(_) then we
    // add it to known program state.
    let maybe_evaluated_expr: Option<u16> = eval_expr(&assign.exp, &state.known);

    // The expression has only Literals and may be immediately
    // evaluated.
    if let Some(e) = maybe_evaluated_expr {
        state.known.insert(assign.id.to_owned(), e);
        assign.val = Some(e);
        return true;
    }

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
    // assignments.sort();

    let mut last_evaluations;
    let mut this_evaluations;

    loop {
        this_evaluations = 0;
        for a in assignments.iter_mut() {
            if eval(a, &mut state) {
                this_evaluations += 1;
            }
        }
        last_evaluations = this_evaluations;
        if this_evaluations == 0 && last_evaluations == 0 {
            break;
        }
    }

    println!("The value of a: {:?}", state.known.get("a"));
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

        my_assign = parse("8 AND r -> j").unwrap();
        eval(&my_assign, &mut state);

        my_assign = parse("z OR r -> t").unwrap();
        eval(&my_assign, &mut state);

        assert_eq!(state.known.get("x"), Some(1).as_ref());
        assert_eq!(state.known.get("z"), Some(1).as_ref());
        assert_eq!(state.known.get("r"), Some(65534).as_ref());
        assert_eq!(state.known.get("s"), Some(0).as_ref());
        assert_eq!(state.known.get("j"), Some(8).as_ref());
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

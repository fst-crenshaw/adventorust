use std::collections::HashMap;

/// The program state is the set of variables whose values are known
/// and the set of variables whose assigned expressions cannot yet be
/// evaluated.
#[derive(Debug)]
pub struct State {
    pub known: HashMap<String, u16>,
}

impl State {
    pub fn new() -> Self {
        Self {
            known: HashMap::new(),
        }
    }
}

/// A term in an expression is one of a variable (like "x") or an
/// unsigned integer (like 1).
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Term {
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
pub enum Exp {
    Literal(u16),
    Variable(String),
    UnaryExp(fn(a: u16) -> u16, Term),
    BinaryExp(fn(a: u16, b: u16) -> u16, Term, Term),
}

/// An Assignment is an identifier and an expression.
///    i.e., <exp> -> <id>
///
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Assignment {
    pub exp: Exp,
    pub id: String,
    pub val: Option<u16>,
}

//! AST stands for Abstract Syntax Tree. It represents the general structure of a program. Given a
//! syntax tree, one can properly reduce and compute the Funny program.

#![allow(dead_code)]

/// Expressions make up the main functional part of the language.
pub enum Expr {
    Literal(Atom),                     // "hello world"
    Operator(Op),                      // 1 + 2
    Identifier(String),                // sum
    Lambda(String, Box<Expr>),         // a b . a
    Application(Box<Expr>, Box<Expr>), // sum list
}

/// Operators on Atoms.
pub enum Op {
    // Arithmetic
    Plus(Atom, Atom),  // `+`
    Minus(Atom, Atom), // `-`
    Mult(Atom, Atom),  // `*`
    Div(Atom, Atom),   // `/`
    Fract(Atom, Atom), // `/.`

    // Comparisons
    Eql(Atom, Atom), // `==`
    Neq(Atom, Atom), // `/=`
    Lt(Atom, Atom),  // `<`
    Gt(Atom, Atom),  // `>`
    Leq(Atom, Atom), // `<=`
    Geq(Atom, Atom), // `>=`

    // Lists
    Cons(Atom, Atom),   // `:`
    Concat(Atom, Atom), // `++`
}

/// Atoms are primitive types that cannot be derived from any other types.
pub enum Atom {
    Byte(u8),   // b'a'
    Char(char), // 'b'
    Int(i64),   // 42
    Float(f64), // 3.14
}

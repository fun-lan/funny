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

/// Atoms are primitive types that cannot be derived from any other types.
pub enum Atom {
    Str(Vec<char>), // "hello world"
}

/// Operators on Atoms.
pub enum Op {
    Concat(Atom, Atom), // `++`
}

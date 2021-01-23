//! AST stands for Abstract Syntax Tree. It represents the general structure of a program. Given a
//! syntax tree, one can properly reduce and compute the Funny program.

#![allow(dead_code)]

/// Statement is a piece of code that adds context to the program. For example, global function
/// alias declaration simply adds a new name to the current package.
pub enum Statement {
    FunAlias(Identifier, Expr), // main = "hello world"
}

/// Expressions make up the main functional part of the language.
pub enum Expr {
    Literal(Atom), // "hello world"
}

/// Atoms are primitive types that cannot be derived from any other types.
pub enum Atom {
    Str(Vec<char>), // "hello world"
}

pub enum Identifier {
    Value(String), // is_even
    Type(String),  // List
}

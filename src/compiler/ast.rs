//! AST stands for Abstract Syntax Tree. It represents the general structure of a program. Given a
//! syntax tree, one can properly reduce and compute the Funny program.

/// Funny program is just a list of statements (e.g. type and function declarations). Hence, we
/// introduce this type alias for explicitness.
pub type Program = Vec<Declaration>;

/// Declaration is a piece of code that adds context to the program. For example, global function
/// alias declaration simply adds a new name to the current package.
pub enum Declaration {
    Function(Identifier, Expression), // main = "hello world"
}

/// Identifier names a type or serves as alias to some value.
pub enum Identifier {
    Value(String), // is_even
    Type(String),  // List
}

/// Expressions make up the main functional part of the language.
pub enum Expression {
    Literal(Atom), // "hello world"
}

/// Atoms are indivisible types that cannot be derived from any other types.
pub enum Atom {
    String(String), // "hello world"
}

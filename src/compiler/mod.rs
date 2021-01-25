//! > *To compile* is to transform plain source code of a Funny program into the Funny AST. The AST
//! will be passed to Funny runtime system for execution.
//!
//! We do so in following steps:
//!
//! 1. `source: String     -> LEXER  -> tokens: Vec<Token>`
//! 2. `tokens: Vec<Token> -> PARSER -> ast: AST`

#![allow(dead_code)]

mod ast;
mod lexer;
mod parser;
mod syntax;
mod token;

/// Use me to compile your Funny source code and produce AST!
pub fn compile(source: String) -> Result<ast::Program, idioma::Error> {
    parser::parse(lexer::tokenize(source)?)
}

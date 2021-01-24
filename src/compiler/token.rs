#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Punctuation(char),
    Literal(String),
}

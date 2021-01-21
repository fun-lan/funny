#![allow(dead_code)]

use std::fmt::Display;

const EOF: &str = "end of file";

/// Character stream powers lexer's ability to go scan source code.
pub struct CharStream {
    stream: Vec<char>,
    length: usize,
    pos: usize,
    col: usize,
    row: usize,
}

impl CharStream {
    /// Construct CharStream from an input String.
    pub fn new(input: String) -> Self {
        let stream: Vec<char> = input.chars().collect();
        let length = stream.len();
        CharStream {
            stream,
            length,
            pos: 0,
            col: 1,
            row: 1,
        }
    }

    /// Peek current character.
    pub fn peek(&self) -> Result<char, idioma::Error> {
        if self.eof() {
            Err(idioma::error(EOF))
        } else {
            Ok(self.this())
        }
    }

    /// Shift reader head one char to the right.
    pub fn shift(&mut self) -> Result<(), idioma::Error> {
        if self.eof() {
            Err(idioma::error(EOF))
        } else {
            let (row, col) = self.newline();
            self.row = row;
            self.col = col;
            self.pos += 1;
            Ok(())
        }
    }

    /// Construct an error with current cursor position.
    pub fn cry<T>(&self, msg: T) -> idioma::Error
    where
        T: Display,
    {
        idioma::error(format!("[{}:{}] {}", self.row, self.col, msg))
    }

    // Take care of position on newlines.
    fn newline(&self) -> (usize, usize) {
        if self.this() == '\n' {
            (self.row + 1, 1)
        } else {
            (self.row, self.col + 1)
        }
    }

    // Detect EOF.
    fn eof(&self) -> bool {
        self.pos == self.length
    }

    // Current char under the reader head.
    fn this(&self) -> char {
        self.stream[self.pos]
    }
}

#[cfg(test)]
mod char_stream_tests {
    use super::*;

    #[test]
    fn peek_and_shift() -> Result<(), idioma::Error> {
        let mut cs = CharStream::new(String::from("he\nl"));
        assert_eq!('h', cs.peek()?, "peek failed");
        cs.shift()?;
        assert_eq!('e', cs.peek()?, "shift failed");
        cs.shift()?;
        assert_eq!('\n', cs.peek()?, "newline skipped");
        cs.shift()?;
        assert_eq!('l', cs.peek()?, "peek or shift failed");
        assert_eq!(1, cs.col, "invlaid column");
        assert_eq!(2, cs.row, "invlaid row");
        assert_eq!(3, cs.pos, "invalid position");
        cs.shift()?;
        if let Ok(_) = cs.peek() {
            panic!("EOF expected but not found");
        }
        if let Ok(_) = cs.shift() {
            panic!("EOF expected but not found");
        }
        Ok(())
    }
}

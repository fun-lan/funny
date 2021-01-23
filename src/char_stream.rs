#![allow(dead_code)]

#[derive(Debug, PartialEq)]
enum Status {
    Offline,
    Online,
}

/// Character stream powers lexer's ability to scan source code.
pub struct CharStream<I: Iterator<Item = (usize, char)>> {
    stream: I,
    status: Status,
    chr: char,
    pos: usize,
    col: usize,
    row: usize,
}

use Status::*;
impl<I> CharStream<I>
where
    I: Iterator<Item = (usize, char)>,
{
    pub fn new(input: I) -> Self {
        let mut cs = CharStream {
            stream: input,
            status: Online,
            chr: '\0',
            pos: 0,
            col: 0,
            row: 1,
        };
        cs.next();
        cs
    }

    /// Request the next `char` from stream. This function keeps track of position, row, and column
    /// at which we're reading.
    pub fn next(&mut self) -> Option<char> {
        if let Some((pos, chr)) = self.stream.next() {
            self.pos = pos;
            self.chr = chr;
            self.handle_newline_feed(chr);
            Some(chr)
        } else {
            self.status = Offline;
            None
        }
    }

    /// Peek at the current `char`. Always returns `None` if `CharStream` is `Offline`.
    pub fn peek(&self) -> Option<char> {
        match self.status {
            Online => Some(self.chr),
            Offline => None,
        }
    }

    /// EOF check. Returns `true` if iterator has been exhausted.
    pub fn eof(&self) -> bool {
        self.status == Offline
    }

    // Manage row and col based on the char received.
    fn handle_newline_feed(&mut self, chr: char) {
        if chr == '\r' {
            () // Do not change anything.
        } else if chr == '\n' {
            self.row += 1;
            self.col = 0;
        } else {
            self.col += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_all_with_position() {
        let mut cs = CharStream::new("hel\r\nlo".char_indices());
        assert_eq!('h', cs.peek().unwrap());
        assert_eq!(0, cs.pos);
        assert_eq!('e', cs.next().unwrap());
        assert_eq!(1, cs.pos);
        assert_eq!('e', cs.peek().unwrap());
        assert_eq!('l', cs.next().unwrap());
        assert_eq!(1, cs.row);
        assert_eq!(2, cs.pos);
        assert_eq!(3, cs.col);
        assert_eq!('\r', cs.next().unwrap());
        assert_eq!(1, cs.row);
        assert_eq!(3, cs.pos);
        assert_eq!(3, cs.col);
        assert_eq!('\n', cs.next().unwrap());
        assert_eq!(2, cs.row);
        assert_eq!(4, cs.pos);
        assert_eq!(0, cs.col);
        assert_eq!('l', cs.next().unwrap());
        assert_eq!('o', cs.next().unwrap());
        assert_eq!(None, cs.next());
        assert_eq!(Offline, cs.status);
    }
}

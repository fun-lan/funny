use super::syntax;
use super::token::Token;

/// Use me to convert source code into a list of tokens!
pub fn tokenize(source: String) -> Result<Vec<Token>, idioma::Error> {
    let chars: Vec<char> = source.chars().collect();
    let progress: Progress = (&chars, 0);
    lex_all(progress)
}

/// Progress type shows us where we are in the lexing process.
type Progress<'a> = (&'a Vec<char>, usize);

/// Return `Progress` with cursor moved one `char` to the right.
fn next(progress: Progress) -> Progress {
    (progress.0, progress.1 + 1)
}

/// Get `char` under cursor from `Progress`.
fn current_char(progress: Progress) -> Option<char> {
    let (source, position) = progress;
    if position >= source.len() {
        None
    } else {
        Some(source[position])
    }
}

/// Test `Progress` on EOF.
fn eof(progress: Progress) -> bool {
    let (source, position) = progress;
    position >= source.len()
}

/// Return `true` on EOF, otherwise apply predicate to `current_char`.
fn check_current_char_or_true(progress: Progress, predicate: fn(char) -> bool) -> bool {
    if let Some(current) = current_char(progress) {
        predicate(current)
    } else {
        true
    }
}

/// In case `check_current_char_or_true` returns `true`, return `progress` unchanged, otherwise,
/// apply `transform` and return the result.
fn check_current_char_or_do(
    progress: Progress,
    predicate: fn(char) -> bool,
    transform: fn(Progress) -> Progress,
) -> Progress {
    if check_current_char_or_true(progress, predicate) {
        progress
    } else {
        transform(progress)
    }
}

/// Skips to newline, consuming the newline char itself.
fn skip_to_newline(progress: Progress) -> Progress {
    check_current_char_or_do(progress, |c| c == '\n', |p| skip_to_newline(next(p)))
}

/// An auxiliary function used by `tokenize` to retrieve all tokens from the input `String`.
fn lex_all(_progress: Progress) -> Result<Vec<Token>, idioma::Error> {
    todo!("sharpvik")
}

/// Skip all whitespace and comments to get to the significant bits.
fn skip_whitespace_and_comments(progress: Progress) -> Progress {
    check_current_char_or_do(
        progress,
        |c| !(c.is_ascii_whitespace() || c == syntax::COMMENT_STARTER),
        |p| skip_whitespace_and_comments(skip_comments(skip_whitespace(p))),
    )
}

/// Skip all whitespace.
fn skip_whitespace(progress: Progress) -> Progress {
    check_current_char_or_do(
        progress,
        |c| !c.is_ascii_whitespace(),
        |p| skip_whitespace(next(p)),
    )
}

/// Skip all comments.
fn skip_comments(progress: Progress) -> Progress {
    check_current_char_or_do(
        progress,
        |c| c != syntax::COMMENT_STARTER,
        |p| skip_comments(skip_to_newline(p)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_answer(input: &str, f: fn(Progress) -> Progress, expect: usize) {
        let vector: Vec<char> = input.to_string().chars().collect();
        let init: Progress = (&vector, 0);
        let (_, answer) = f(init);
        assert_eq!(expect, answer, "shoul've skipped to #{}", expect);
    }

    #[test]
    fn can_skip_whitespace_and_comments() {
        check_answer(
            " \t\r\n# comment \r\n#\t code\n \r\n\tmain = 42  ",
            skip_whitespace_and_comments,
            28,
        );
        // destructive
        check_answer(
            " \t\r\n# comment \r\n#\t code\n \r\n\t",
            skip_whitespace_and_comments,
            28,
        );
        check_answer("main = 42", skip_whitespace_and_comments, 0);
        check_answer(" = 42", skip_whitespace_and_comments, 1);
    }
}

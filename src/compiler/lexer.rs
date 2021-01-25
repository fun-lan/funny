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

/// Skips to newline, consuming the newline char itself.
fn skip_to_newline(progress: Progress) -> Progress {
    if let Some(current) = current_char(progress) {
        if current == '\n' {
            next(progress)
        } else {
            skip_to_newline(next(progress))
        }
    } else {
        progress
    }
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

/// An auxiliary function used by `tokenize` to retrieve all tokens from the input `String`.
fn lex_all(_progress: Progress) -> Result<Vec<Token>, idioma::Error> {
    todo!("sharpvik")
}

/// Skip all whitespace and comments to get to the significant bits.
fn skip_whitespace_and_comments(progress: Progress) -> Progress {
    if eof(progress) {
        progress
    } else if whitespace_and_comments_skipped(progress) {
        progress
    } else {
        let no_whitespace: Progress = skip_whitespace(progress);
        let no_comment: Progress = skip_comments(no_whitespace);
        skip_whitespace_and_comments(no_comment)
    }
}

/// Check that all whitespace and comments are skipped. This is a helper function that makes sure
/// that we catch all base cases in `skip_whitespace_and_comments`.
fn whitespace_and_comments_skipped(progress: Progress) -> bool {
    if let Some(current) = current_char(progress) {
        !(current.is_ascii_whitespace() || current == syntax::COMMENT_STARTER)
    } else {
        true
    }
}

/// Skip all whitespace.
fn skip_whitespace(progress: Progress) -> Progress {
    if let Some(current) = current_char(progress) {
        if current.is_ascii_whitespace() {
            skip_whitespace(next(progress))
        } else {
            progress
        }
    } else {
        progress
    }
}

/// Skip all comments.
fn skip_comments(progress: Progress) -> Progress {
    if let Some(current) = current_char(progress) {
        if current == syntax::COMMENT_STARTER {
            skip_comments(skip_to_newline(progress))
        } else {
            progress
        }
    } else {
        progress
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_answer(input: &str, f: fn(Progress) -> Progress, expect: usize) {
        let vector: Vec<char> = String::from(input).chars().collect();
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

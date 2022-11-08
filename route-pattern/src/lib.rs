#[cfg(feature = "handparser")]
mod hand_parser;
#[cfg(feature = "with-nom")]
mod nom_parser;
use regex::Regex;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ParseError {
    #[error("Cannot parse expression")]
    ParseError(String),
    #[error("Embedded regex cannot compile")]
    BadRegex(String),
}

#[cfg(feature = "handparser")]
pub fn compile_impl(pattern: &str, dstart: char, dend: char) -> Result<Regex, ParseError> {
    let parsed = hand_parser::parse(pattern, dstart, dend)
        .map_err(|err| ParseError::ParseError(err.to_string()))?;
    let mut out = String::new();

    for exp in parsed {
        match exp {
            hand_parser::Token::Regex(re) => out.push_str(&re),
            hand_parser::Token::String(s) => {
                out.push_str(&regex::escape(&s));
            }
        }
    }

    out = format!("^{}$", out);
    Regex::new(&out).map_err(|err| ParseError::BadRegex(err.to_string()))
}

#[cfg(feature = "with-nom")]
fn compile_impl(pattern: &str, dstart: char, dend: char) -> Result<Regex, ParseError> {
    let (rest, parsed) = nom_parser::parse(pattern, dstart, dend)
        .map_err(|err| ParseError::ParseError(err.to_string()))?;
    if !rest.is_empty() {
        return Err(ParseError::ParseError(format!(
            "input not consumed entirely: {}",
            rest
        )));
    }
    let mut out = String::new();

    for exp in parsed {
        match exp {
            nom_parser::Token::Regex(re) => out.push_str(re),
            nom_parser::Token::String(s) => {
                out.push_str(&regex::escape(s));
            }
        }
    }
    out = format!("^{}$", out);
    Regex::new(&out).map_err(|err| ParseError::BadRegex(err.to_string()))
}

/// Compile a route pattern. Get back a `Regex` which you can use as you see fit.
///
/// # Errors
///
/// This function will return an error if parsing or regex compilation fails
pub fn compile(pattern: &str, dstart: char, dend: char) -> Result<Regex, ParseError> {
    compile_impl(pattern, dstart, dend)
}
/// Match a route pattern.
///
/// # Errors
///
/// This function will return an error if parsing or regex compilation fails
pub fn is_match(pattern: &str, dstart: char, dend: char, text: &str) -> Result<bool, ParseError> {
    let re = compile(pattern, dstart, dend)?;

    Ok(re.is_match(text))
}
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn match_glob() {
        assert_eq!(is_match("foo/<.*>", '<', '>', "foo/bar").unwrap(), true);
    }

    #[test]
    fn match_nested() {
        assert_eq!(
            is_match("foo/{a{1,4}}/", '{', '}', "foo/aaa/").unwrap(),
            true
        );
    }

    #[test]
    fn dont_match_nested() {
        assert_eq!(
            is_match("foo/{a{1,4}}/", '{', '}', "foo/aaaaaaaa/").unwrap(),
            false
        );
    }

    #[test]
    fn match_multiple() {
        assert_eq!(
            is_match("foo/{b{1,4}}/{[0-9]+}", '{', '}', "foo/bbb/123").unwrap(),
            true
        );
    }
}

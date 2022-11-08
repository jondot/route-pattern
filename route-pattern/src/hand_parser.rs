use std::{iter::Peekable, str::Chars};

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ParseError {
    #[error("No ending delimiter found")]
    DelimiterMissing,
}

#[derive(Debug)]
pub enum Token {
    String(String),
    Regex(String),
}

fn expr_regex(
    iter: &mut Peekable<Chars<'_>>,
    dstart: char,
    dend: char,
) -> Result<Token, ParseError> {
    let mut s = String::new();
    let mut braces = 0;

    while let Some(c) = iter.peek() {
        match c {
            c if dstart.eq(c) => {
                braces += 1;
                if braces > 1 {
                    s.push(*c);
                }
            }
            c if dend.eq(c) => {
                braces -= 1;
                if braces == 0 {
                    iter.next();
                    return Ok(Token::Regex(s));
                }
                s.push(*c);
            }
            c => {
                s.push(*c);
            }
        }
        iter.next();
    }
    Err(ParseError::DelimiterMissing)
}

fn expr_str(iter: &mut Peekable<Chars<'_>>, until: char) -> Token {
    let mut s = String::new();
    while let Some(c) = iter.peek() {
        match c {
            c if until.eq(c) => {
                return Token::String(s);
            }
            c => {
                s.push(*c);
            }
        }

        iter.next();
    }
    Token::String(s)
}

fn parse_impl(pattern: &str, dstart: char, dend: char) -> Result<Vec<Token>, ParseError> {
    let mut iter = pattern.chars().peekable();
    let mut tokens = vec![];
    while let Some(c) = iter.peek() {
        match c {
            c if dstart.eq(c) => {
                let re_token = expr_regex(&mut iter, dstart, dend)?;
                tokens.push(re_token);
            }
            _ => {
                let tok = expr_str(&mut iter, dstart);
                tokens.push(tok);
            }
        }
    }
    Ok(tokens)
}

/// Parse a route pattern
///
/// # Errors
///
/// This function will return an error if parsing failed
pub fn parse(pattern: &str, dstart: char, dend: char) -> Result<Vec<Token>, ParseError> {
    if pattern.is_empty() {
        return Ok(vec![]);
    }

    parse_impl(pattern, dstart, dend)
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_yaml_snapshot;
    use pretty_assertions::assert_eq;
    fn serialize(res: &[Token]) -> Vec<String> {
        res.iter()
            .map(|t| match t {
                Token::String(s) => "text:".to_string() + s,
                Token::Regex(s) => "regex:".to_string() + s,
            })
            .collect::<Vec<_>>()
    }
    #[test]
    fn empty_tokens() {
        assert_yaml_snapshot!(serialize(&parse("", '<', '>').unwrap()));
    }
    #[test]
    fn no_regex() {
        assert_yaml_snapshot!(serialize(&parse("abc", '<', '>').unwrap()));
    }
    #[test]
    fn no_string() {
        assert_yaml_snapshot!(serialize(&parse("<abc.+>", '<', '>').unwrap()));
    }

    #[test]
    fn it_works() {
        assert_yaml_snapshot!(serialize(
            &parse("/articles/<.+>/update", '<', '>').unwrap()
        ));
    }

    #[test]
    fn nested_pattern_first_level_only() {
        assert_yaml_snapshot!(serialize(
            &parse("/articles/<a<.+>b>/update", '<', '>').unwrap()
        ));
    }
    #[test]
    fn bad_parens_balance() {
        let tokens = parse("/articles/<a<.+b>/update", '<', '>');
        assert_eq!(tokens.unwrap_err(), ParseError::DelimiterMissing);
    }
    #[test]
    fn bad_parens_balance_2() {
        let tokens = parse("/articles/<a.+b/update", '<', '>');
        assert_eq!(tokens.unwrap_err(), ParseError::DelimiterMissing);
    }
    #[test]
    fn multiple_groups() {
        assert_yaml_snapshot!(serialize(
            &parse("/ar<\\S+>/<.+>/update", '<', '>').unwrap()
        ));
    }
}

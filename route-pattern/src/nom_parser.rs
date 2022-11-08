use nom::bytes::complete::take_till1;
use nom::combinator::{self};
use nom::{
    branch::alt,
    bytes::complete::take_while1,
    character::complete,
    error::{ErrorKind, ParseError},
    multi::many0,
    sequence::delimited,
    Err, IResult,
};
use nom_supreme::error::{ErrorTree, GenericErrorTree};
use nom_supreme::parser_ext::ParserExt;

#[derive(Debug)]
pub enum Token<'a> {
    String(&'a str),
    Regex(&'a str),
}

fn text(input: &'_ str) -> IResult<&'_ str, &'_ str, ErrorTree<&str>> {
    // watch out: `while` (as opposed to `while1`) returns an empty match, which goes back
    // as input to many0, which will again happily match empty, and again...
    // in order for a parser to progress along the input, we must seek and avoid
    // matching positive for empty input
    take_while1(|_| true)(input)
}
fn take_until_unbalanced(
    opening_bracket: char,
    closing_bracket: char,
) -> impl Fn(&str) -> IResult<&str, &str, ErrorTree<&str>> {
    move |i: &str| {
        let mut index = 0;
        let mut bracket_counter = 1; // we saw one already
        let iter = i.chars();
        for c in iter {
            match c {
                c if c == opening_bracket => {
                    bracket_counter += 1;
                }
                c if c == closing_bracket => {
                    bracket_counter -= 1;
                }
                _ => {}
            };
            if bracket_counter == 0 {
                return Ok((&i[index..], &i[..index]));
            }
            index += 1;
        }

        println!("error {} {}", &i[index..], &i[..index]);
        Err(Err::Error(GenericErrorTree::from_error_kind(
            i,
            ErrorKind::TakeUntil,
        )))
    }
}

pub fn parse(
    input: &'_ str,
    dstart: char,
    dend: char,
) -> IResult<&'_ str, Vec<Token<'_>>, ErrorTree<&str>> {
    many0(
        alt((
            combinator::map(take_till1(|c| c == dstart), Token::String).context("until a paren"),
            combinator::map(
                delimited(
                    complete::char(dstart),
                    take_until_unbalanced(dstart, dend).context("balanced parens"),
                    complete::char(dend),
                )
                .context("parens"),
                Token::Regex,
            ),
            combinator::map(text, Token::String).context("regular text"),
        ))
        .context("either one of"),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_yaml_snapshot;
    use nom::Finish;

    fn serialize(data: (&str, Vec<Token<'_>>)) -> (String, Vec<String>) {
        let (inp, res) = data;
        (
            format!("remaining:[{}]", inp),
            res.iter()
                .map(|t| match t {
                    Token::String(s) => "text:".to_string() + s,
                    Token::Regex(s) => "regex:".to_string() + s,
                })
                .collect::<Vec<_>>(),
        )
    }

    #[test]
    fn empty() {
        assert_yaml_snapshot!(parse("", '<', '>').finish().map(serialize).unwrap());
    }

    #[test]
    fn no_parens() {
        assert_yaml_snapshot!(parse("foobar", '<', '>').finish().map(serialize).unwrap());
    }

    #[test]
    fn no_text() {
        assert_yaml_snapshot!(parse("<foobar>", '<', '>').finish().map(serialize).unwrap());
    }

    #[test]
    fn empty_parens() {
        assert_yaml_snapshot!(parse("foobar<>", '<', '>').finish().map(serialize).unwrap());
    }

    #[test]
    fn bad_parens_balance() {
        assert_yaml_snapshot!(parse("foobar<<yo>", '<', '>')
            .finish()
            .map(serialize)
            .unwrap());
        assert_yaml_snapshot!(parse("foobar<yo>>", '<', '>')
            .finish()
            .map(serialize)
            .unwrap());
    }

    #[test]
    fn with_nested() {
        assert_yaml_snapshot!(parse("<re1>foobar<a<1,5>foo>foo3<re11>", '<', '>')
            .finish()
            .map(serialize)
            .unwrap());
    }

    #[test]
    fn with_curlies() {
        assert_yaml_snapshot!(parse("/foo/{a{1,2}-.+}/update", '{', '}')
            .finish()
            .map(serialize)
            .unwrap());
    }
}

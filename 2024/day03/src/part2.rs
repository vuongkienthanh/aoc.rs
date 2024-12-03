use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, digit1},
    combinator::value,
    multi::{many1, many_till},
    sequence::{delimited, preceded, separated_pair},
    IResult, Parser,
};

#[derive(Clone, Debug)]
enum Cmd {
    Do,
    Dont,
    Mul(usize, usize),
}

fn cmddo(input: &str) -> IResult<&str, Cmd> {
    value(Cmd::Do, tag("do()"))(input)
}
fn cmddont(input: &str) -> IResult<&str, Cmd> {
    value(Cmd::Dont, tag("don't()"))(input)
}
fn cmdmul(input: &str) -> IResult<&str, Cmd> {
    preceded(
        tag("mul"),
        delimited(
            tag("("),
            separated_pair(
                digit1.map(|a: &str| a.parse::<usize>().unwrap()),
                tag(","),
                digit1.map(|a: &str| a.parse::<usize>().unwrap()),
            )
            .map(|(a, b)| Cmd::Mul(a, b)),
            tag(")"),
        ),
    )(input)
}
fn parse(input: &str) -> IResult<&str, Vec<Cmd>> {
    many1(many_till(anychar, alt((cmddo, cmddont, cmdmul))).map(|x| x.1))(input)
}
pub fn process(_input: &str) -> usize {
    let (_, v) = parse(_input).unwrap();
    v.into_iter()
        .fold((0, true), |(sum, is_enabled), ele| match ele {
            Cmd::Do => (sum, true),
            Cmd::Dont => (sum, false),
            Cmd::Mul(a, b) => (sum + if is_enabled { a * b } else { 0 }, is_enabled),
        })
        .0
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;
        assert_eq!(process(input), 48);
    }
}

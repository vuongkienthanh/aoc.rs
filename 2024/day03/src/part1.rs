use nom::{
    bytes::complete::tag,
    character::complete::{anychar, digit1},
    multi::{many1, many_till},
    sequence::{delimited, preceded, separated_pair},
    IResult, Parser,
};

fn mul(input: &str) -> IResult<&str, (usize, usize)> {
    preceded(
        tag("mul"),
        delimited(
            tag("("),
            separated_pair(
                digit1.map(|a: &str| a.parse::<usize>().unwrap()),
                tag(","),
                digit1.map(|a: &str| a.parse::<usize>().unwrap()),
            ),
            tag(")"),
        ),
    )(input)
}
fn parse(input: &str) -> IResult<&str, Vec<(usize, usize)>> {
    many1(many_till(anychar, mul).map(|x| x.1))(input)
}

pub fn process(_input: &str) -> usize {
    let (_, v) = parse(_input).unwrap();
    v.into_iter().map(|(a, b)| a * b).sum()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
        assert_eq!(process(input), 161);
    }
}

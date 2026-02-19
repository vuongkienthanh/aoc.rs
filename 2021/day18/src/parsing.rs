use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{delimited, separated_pair},
};


#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Item {
    Literal(usize),
    Number(Number),
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Number {
    pub left: Box<Item>,
    pub right: Box<Item>,
}

fn parse_item(input: &str) -> IResult<&str, Item> {
    alt((
        complete::usize.map(Item::Literal),
        parse_number.map(Item::Number),
    ))
    .parse(input)
}

pub fn parse_number(input: &str) -> IResult<&str, Number> {
    delimited(
        tag("["),
        separated_pair(parse_item, tag(","), parse_item),
        tag("]"),
    )
    .map(|(a, b)| Number {
        left: Box::new(a),
        right: Box::new(b),
    })
    .parse(input)
}

pub fn parse_input(input: &str) -> Vec<Number> {
    all_consuming(separated_list1(line_ending, parse_number))
        .parse(input)
        .unwrap()
        .1
}


#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]")]
    #[case("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]")]
    #[case("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]")]
    #[case(
        "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
        "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
    )]
    #[case("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[7,0]]]]")]
    fn test_explode(#[case] input: &str, #[case] expected: &str) {
        let (_, mut input) = parse_number(input).unwrap();
        input.explode(0);
        let (_, expected) = parse_number(expected).unwrap();
        assert_eq!(input, expected);
    }

    #[rstest]
    #[case(
        "[[[[4,3],4],4],[7,[[8,4],9]]]",
        "[1,1]",
        "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
    )]
    #[case(
        "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
        "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
        "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"
    )]
    fn test_add(#[case] a: &str, #[case] b: &str, #[case] expected: &str) {
        let (_, a) = parse_number(a).unwrap();
        let (_, b) = parse_number(b).unwrap();
        let ans = a.add(b);
        let (_, expected) = parse_number(expected).unwrap();
        assert_eq!(ans, expected);
    }
}

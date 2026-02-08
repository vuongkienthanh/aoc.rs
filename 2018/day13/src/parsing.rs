use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::{many1, separated_list1},
};

pub use aoc_helper::direction::Direction;
use fxhash::FxHashMap;
use nom_locate::LocatedSpan;

type Span<'a> = LocatedSpan<&'a str>;
pub type Cart = (usize, usize, Direction, usize);
pub type Map = Vec<Vec<Path>>;
pub type Carts = FxHashMap<(usize, usize), (Direction, usize)>;

#[derive(Debug, Eq, PartialEq)]
pub enum Path {
    Straight,
    Cross,
    Forward,
    Backward,
    Blank,
}

fn parse_cart(input: Span) -> IResult<Span, Cart> {
    let (row, col) = (input.location_line(), input.get_column());
    let (input, cart) = alt((
        complete::char('^').map(|_| Direction::Up),
        complete::char('v').map(|_| Direction::Down),
        complete::char('<').map(|_| Direction::Left),
        complete::char('>').map(|_| Direction::Right),
    ))
    .parse(input)?;
    Ok((input, (row as usize - 1, col - 1, cart, 0)))
}

fn parse_path(input: Span) -> IResult<Span, Path> {
    alt((
        complete::char('-').map(|_| Path::Straight),
        complete::char('|').map(|_| Path::Straight),
        complete::char('+').map(|_| Path::Cross),
        complete::char('/').map(|_| Path::Forward),
        complete::char('\\').map(|_| Path::Backward),
        complete::char(' ').map(|_| Path::Blank),
    ))
    .parse(input)
}

fn parse_single_char(input: Span) -> IResult<Span, (Path, Option<Cart>)> {
    alt((
        parse_path.map(|p| (p, None)),
        parse_cart.map(|c| (Path::Straight, Some(c))),
    ))
    .parse(input)
}

fn parse_line(input: Span) -> IResult<Span, (Vec<Path>, Vec<Cart>)> {
    let (input, v) = many1(parse_single_char).parse(input)?;
    let (line, carts) = v
        .into_iter()
        .fold((vec![], vec![]), |(mut line, mut carts), (p, c)| {
            line.push(p);
            if let Some(c) = c {
                carts.push(c);
            }
            (line, carts)
        });
    Ok((input, (line, carts)))
}

pub fn parse_input(input: &str) -> (Map, Carts) {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(Span::new(input))
        .unwrap()
        .1
        .into_iter()
        .fold(
            (vec![], FxHashMap::default()),
            |(mut a, mut b), (vp, vc)| {
                a.push(vp);
                for (row, col, dir, turn) in vc {
                    b.insert((row, col), (dir, turn));
                }
                (a, b)
            },
        )
}

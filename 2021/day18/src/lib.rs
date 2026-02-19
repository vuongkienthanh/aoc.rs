pub mod parsing;
pub mod part1;
pub mod part2;

use parsing::{Item, Number};
use std::fmt;
impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Item::Literal(n) => write!(f, "{n}"),
            Item::Number(n) => write!(f, "{n}"),
        }
    }
}
impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{},{}]", self.left, self.right)
    }
}

#[derive(Debug)]
pub enum ExplodeResult {
    Origin(usize, usize),
    Left(usize),
    Right(usize),
    Done,
    No,
}

impl Number {
    pub fn add(self, other: Number) -> Number {
        let mut ans = Number {
            left: Box::new(Item::Number(self)),
            right: Box::new(Item::Number(other)),
        };
        loop {
            let same = matches!(ans.explode(0), ExplodeResult::No);
            if same && !ans.split() {
                break;
            }
        }
        ans
    }
    pub fn magnitude(&self) -> usize {
        self.left.magnitude() * 3 + self.right.magnitude() * 2
    }
    pub fn explode(&mut self, lvl: usize) -> ExplodeResult {
        // println!("explode at {lvl}   ==   {self}");
        if lvl > 3
            && let (Item::Literal(a), Item::Literal(b)) = (*self.left.clone(), *self.right.clone())
        {
            // println!("expand [{a},{b}]");
            return ExplodeResult::Origin(a, b);
        }

        match self.left.explode(lvl + 1) {
            ExplodeResult::Origin(a, b) => {
                self.right.append_left(b);
                *self.left = Item::Literal(0);
                return ExplodeResult::Left(a);
            }
            ExplodeResult::Right(i) => {
                self.right.append_left(i);
                return ExplodeResult::Done;
            }
            ExplodeResult::No => (),
            x => return x,
        }
        match self.right.explode(lvl + 1) {
            ExplodeResult::Origin(a, b) => {
                self.left.append_right(a);
                *self.right = Item::Literal(0);
                return ExplodeResult::Right(b);
            }
            ExplodeResult::Left(i) => {
                self.left.append_right(i);
                ExplodeResult::Done
            }
            x => x,
        }
    }
    pub fn split(&mut self) -> bool {
        self.left.split() || self.right.split()
    }
}

impl Item {
    pub fn magnitude(&self) -> usize {
        match self {
            Item::Literal(a) => *a,
            Item::Number(Number { left, right }) => left.magnitude() * 3 + right.magnitude() * 2,
        }
    }
    pub fn append_left(&mut self, i: usize) {
        match self {
            Item::Literal(a) => {
                *a += i;
            }
            Item::Number(n) => {
                n.left.append_left(i);
            }
        }
    }
    pub fn append_right(&mut self, i: usize) {
        match self {
            Item::Literal(a) => {
                *a += i;
            }
            Item::Number(n) => {
                n.right.append_right(i);
            }
        }
    }
    pub fn explode(&mut self, lvl: usize) -> ExplodeResult {
        match self {
            Item::Literal(_) => ExplodeResult::No,
            Item::Number(n) => n.explode(lvl),
        }
    }
    pub fn split(&mut self) -> bool {
        match self {
            Item::Literal(a) => {
                if *a > 9 {
                    // println!("split {a}");
                    *self = Item::Number(Number {
                        left: Box::new(Item::Literal(*a / 2)),
                        right: Box::new(Item::Literal(*a - *a / 2)),
                    });
                    true
                } else {
                    false
                }
            }
            Item::Number(n) => n.split(),
        }
    }
}

pub fn add_all(input: Vec<Number>) -> Number {
    let mut i = input.into_iter();
    let mut a = i.next().unwrap();
    while let Some(b) = i.next() {
        a = a.add(b);
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsing::{parse_input, parse_number};
    use rstest::*;

    #[rstest]
    #[case(
        r#"[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]"#,
        "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
    )]
    fn test_add_all(#[case] input: &str, #[case] expected: &str) {
        let input = parse_input(input);
        let ans = add_all(input);
        let (_, expected) = parse_number(expected).unwrap();
        assert_eq!(ans, expected);
    }
}

use crate::is_triangle;
use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);

    let mut i = input.into_iter();
    let mut ans = 0;

    while let Some((a1, a2, a3)) = i.next() {
        let (b1, b2, b3) = i.next().unwrap();
        let (c1, c2, c3) = i.next().unwrap();

        ans += [(a1, b1, c1), (a2, b2, c2), (a3, b3, c3)]
            .into_iter()
            .filter(|x| is_triangle(*x))
            .count();
    }

    ans
}

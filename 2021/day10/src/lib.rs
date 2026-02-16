pub mod part1;
pub mod part2;

fn is_closing_char(c: char) -> bool {
    ['>', ')', '}', ']'].contains(&c)
}
fn matching_closing_char(c: char) -> char {
    match c {
        '<' => '>',
        '(' => ')',
        '{' => '}',
        '[' => ']',
        _ => panic!(),
    }
}

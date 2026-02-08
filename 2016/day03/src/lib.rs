pub mod parsing;
pub mod part1;
pub mod part2;

pub fn is_triangle((a, b, c): (usize, usize, usize)) -> bool {
    (a + b) > c && (a + c) > b && (b + c) > a
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_triangle() {
        assert!(!is_triangle((5, 10, 25)));
    }
}

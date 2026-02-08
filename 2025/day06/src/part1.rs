use crate::parsing::{Item, parse_input};
use grid::Grid;

pub fn process(_input: &str) -> usize {
    let (nums, ops) = parse_input(_input);
    let nums: Grid<usize> = nums.into();

    let mut ans = 0;
    for (col, op) in nums.iter_cols().zip(ops) {
        match op {
            Item::Add => ans += col.sum::<usize>(),
            Item::Mul => ans += col.product::<usize>(),
        }
    }

    ans
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 4277556);
    }
}

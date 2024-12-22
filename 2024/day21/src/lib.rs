pub mod part1;
pub mod part2;

mod enums;
pub use enums::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore = "reason"]
    fn test_process() {
        for (pair, actions) in numeric_paths() {
            println!("========================pair = {pair:?}");
            for ac in actions {
                println!("action = {ac:?}");
            }
        }
        assert_eq!(0, 2);
    }
}

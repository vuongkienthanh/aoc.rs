pub mod part1;
pub mod part2;

mod enums;
pub use enums::numeric_paths;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let paths = numeric_paths();
        for (pair, actions) in numeric_paths() {
            println!("========================pair = {pair:?}");
            for ac in actions {
                println!("action = {ac:?}");
            }

        }
        assert_eq!(0, 2);
    }
}

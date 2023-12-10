fn part1(_input: &str) -> String {
    _input
        .lines()
        .filter_map(|line| {
            let mut linesplit = line.split(": ");
            let game_id = linesplit
                .next()
                .unwrap()
                .get(5..)
                .unwrap()
                .parse::<usize>()
                .unwrap();
            if linesplit.next().unwrap().split("; ").all(|set| {
                set.split(", ").all(|cube| {
                    let mut cubesplit = cube.split(' ');
                    let count = cubesplit.next().unwrap().parse::<usize>().unwrap();
                    let color = cubesplit.next().unwrap();
                    match color {
                        "blue" => count <= 14,
                        "green" => count <= 13,
                        "red" => count <= 12,
                        _ => unreachable!(),
                    }
                })
            }) {
                Some(game_id)
            } else {
                None
            }
        })
        .sum::<usize>()
        .to_string()
}
fn part2(_input: &str) -> String {
    _input
        .lines()
        .map(|line| {
            let mut linesplit = line.split(": ");
            let mut bluemax = 0;
            let mut greenmax = 0;
            let mut redmax = 0;
            for set in linesplit.nth(1).unwrap().split("; ") {
                for cube in set.split(", ") {
                    let mut cubesplit = cube.split(' ');
                    let count = cubesplit.next().unwrap().parse::<usize>().unwrap();
                    let color = cubesplit.next().unwrap();
                    match color {
                        "blue" if count > bluemax => bluemax = count,
                        "green" if count > greenmax => greenmax = count,
                        "red" if count > redmax => redmax = count,
                        _ => (),
                    }
                }
            }
            bluemax * redmax * greenmax
        })
        .sum::<usize>()
        .to_string()
}

fn main() {
    let input = include_str!("input.txt");
    // println!("{}", part1(input));
    println!("{}", part2(input));
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore]
    fn test_part1() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        assert_eq!(part1(input), "8");
    }
    #[test]
    // #[ignore]
    fn test_part2() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        assert_eq!(part2(input), "2286");
    }
}

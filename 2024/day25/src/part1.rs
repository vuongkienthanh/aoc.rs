pub fn process(_input: &str) -> usize {
    let mut keys = vec![];
    let mut locks = vec![];
    for block in _input.split("\n\n") {
        if block.lines().next().unwrap() == "#####" {
            let mut lock = [0, 0, 0, 0, 0];
            for line in block.lines().skip(1).take(5) {
                for (i, c) in line.char_indices() {
                    if c == '#' {
                        lock[i] += 1;
                    }
                }
            }
            locks.push(lock);
        } else {
            let mut key = [0, 0, 0, 0, 0];
            for line in block.lines().skip(1).take(5) {
                for (i, c) in line.char_indices() {
                    if c == '#' {
                        key[i] += 1;
                    }
                }
            }
            keys.push(key);
        }
    }
    let mut ans = 0;
    for key in &keys {
        for lock in &locks {
            if (0..5).all(|i| key[i] + lock[i] <= 5) {
                ans += 1;
            }
        }
    }
    ans
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####"#;
        assert_eq!(process(input), 3);
    }
}

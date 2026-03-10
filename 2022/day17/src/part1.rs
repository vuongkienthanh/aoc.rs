//! treat up as right, down as left

const ROCKS: [[[char; 7]; 4]; 5] = [
    [
        ['.', '.', '#', '#', '#', '#', '.'],
        ['.', '.', '.', '.', '.', '.', '.'],
        ['.', '.', '.', '.', '.', '.', '.'],
        ['.', '.', '.', '.', '.', '.', '.'],
    ],
    [
        ['.', '.', '.', '#', '.', '.', '.'],
        ['.', '.', '#', '#', '#', '.', '.'],
        ['.', '.', '.', '#', '.', '.', '.'],
        ['.', '.', '.', '.', '.', '.', '.'],
    ],
    [
        ['.', '.', '#', '#', '#', '.', '.'],
        ['.', '.', '#', '.', '.', '.', '.'],
        ['.', '.', '#', '.', '.', '.', '.'],
        ['.', '.', '.', '.', '.', '.', '.'],
    ],
    [
        ['.', '.', '.', '.', '#', '.', '.'],
        ['.', '.', '.', '.', '#', '.', '.'],
        ['.', '.', '.', '.', '#', '.', '.'],
        ['.', '.', '.', '.', '#', '.', '.'],
    ],
    [
        ['.', '.', '.', '#', '#', '.', '.'],
        ['.', '.', '.', '#', '#', '.', '.'],
        ['.', '.', '.', '.', '.', '.', '.'],
        ['.', '.', '.', '.', '.', '.', '.'],
    ],
];

pub fn process(_input: &str) -> usize {
    let mut hole: Vec<[char; 7]> = vec![['.'; 7]; 3];
    let mut rocks = ROCKS.iter().cycle();
    for i in 0..2 {
        if let Some(last_not_empty) = hole
            .iter()
            .rev()
            .enumerate()
            .find_map(|(i, row)| (!row.iter().all(|x| *x == '.')).then_some(i))
        {
            hole.extend(vec![['.'; 7]; 3 - last_not_empty]);
        }
        let rock = rocks.next().unwrap();
        hole.extend(rock);

        println!("{i}");
        for row in hole.iter().rev() {
            println!("{row:?}");
        }
    }
    
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_() {
        let _input = r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"#;
        assert_eq!(process(_input), 3068);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"#
    }
    
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 3068);
    }
}

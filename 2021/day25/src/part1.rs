use grid::Grid;

pub fn process(_input: &str) -> usize {
    let mut input = Grid::from(
        _input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    );
    let mut i = 0;
    loop {
        i += 1;
        let mut origin = input.clone();
        for r in 0..input.rows() {
            for c in 0..input.cols() {
                if origin[(r, c)] != '>' {
                    continue;
                }
                let dst = (r, (c + 1) % input.cols());
                if origin[dst] == '.' {
                    input.swap((r, c), dst);
                }
            }
        }
        let same = input == origin;
        origin = input.clone();
        for r in 0..input.rows() {
            for c in 0..input.cols() {
                if origin[(r, c)] != 'v' {
                    continue;
                }
                let dst = ((r + 1) % input.rows(), c);
                if origin[dst] == '.' {
                    input.swap((r, c), dst);
                }
            }
        }

        if same && input == origin {
            break i;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 58);
    }
}

use super::{directions, parse, Coord, Grid};

type Memo = ::std::collections::HashMap<Coord, usize>;

pub fn process(_input: &str) -> usize {
    let mut memo = Memo::new();
    let grid = parse(_input);
    grid.indexed_iter()
        .filter(|(_, c)| **c == 0)
        .map(|(x, _)| [x.0, x.1])
        .map(|x| rating(x, &grid, &mut memo))
        .sum()
}

fn rating(x: Coord, grid: &Grid<u32>, memo: &mut Memo) -> usize {
    if grid[x.into()] == 9 {
        1
    } else if let Some(num) = memo.get(&x) {
        *num
    } else {
        let num = directions(x, grid)
            .into_iter()
            .filter(|new_x| grid[(*new_x).into()] == grid[x.into()] + 1)
            .map(|new_x| rating(new_x, grid, memo))
            .sum();
        memo.insert(x, num);
        num
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;
        assert_eq!(process(input), 81);
    }
}

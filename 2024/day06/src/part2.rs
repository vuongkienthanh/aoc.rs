use super::{parse, CellType, ForwardResult, Guard, JumpResult};
use grid::Grid;

pub fn process(_input: &str) -> usize {
    let (mut guard, mut grid) = parse(_input);
    let mut tried_obs = Grid::new(grid.rows(), grid.cols());
    tried_obs.fill(0);
    tried_obs[guard.position.into()] = 1;
    let mut loop_counter = 0;

    loop {
        let ForwardResult {
            middle_path,
            next_guard,
            is_stop,
        } = guard.forward(&grid);

        // for each pair (prv, nxt) coord in start + path + next_guard
        // try putting obs in right hand coord
        // and then make a guard at prv and run to check loop
        for (prv, nxt) in Some(&guard.position)
            .into_iter()
            .chain(middle_path.iter())
            .zip(middle_path.iter().chain(Some(&next_guard.position)))
            .map(|(prv, nxt)| (*prv, *nxt))
            .filter(|(_, nxt)| tried_obs[(*nxt).into()] != 1)
            .collect::<Vec<_>>()
        {
            // ---- do smth with nxt coord
            // mark as tried obs && update grid
            tried_obs[nxt.into()] = 1;
            grid[nxt.into()] = CellType::Obstacle;

            // ---- do smth with prv coord
            // assume a guard that turned before obs
            let mut loop_guard = Guard {
                direction: next_guard.direction.clone(),
                position: prv,
            };
            // dfs algo to check loop
            let mut dfs = Vec::from([loop_guard.clone()]);
            loop {
                let JumpResult {
                    next_guard: next_loop_guard,
                    is_stop,
                } = loop_guard.jump(&grid);
                if is_stop {
                    break;
                } else if dfs.contains(&next_loop_guard) {
                    loop_counter += 1;
                    break;
                } else {
                    dfs.push(next_loop_guard.clone());
                    loop_guard = next_loop_guard;
                }
            }
            // ---- finish do smth with prv coord

            // revert grid to initial state
            grid[nxt.into()] = CellType::Empty;
        }

        // prepare next loop
        if is_stop {
            break;
        }
        guard = next_guard;
    }
    loop_counter
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
        assert_eq!(process(input), 6);
    }
}

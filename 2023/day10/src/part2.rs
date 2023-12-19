use crate::PipeMaze;

pub fn process(_input: &str) -> usize {
    let maze = PipeMaze::new(_input);
    let start_coord = maze.start_coord.clone();
    let start_pipe = maze.start_pipe.clone();

    let maze_coords = maze.collect::<Vec<(usize, usize)>>();

    // change input 'S' to corresponding pipe
    let mut maze_map = _input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    *maze_map
        .get_mut(start_coord.0)
        .unwrap()
        .get_mut(start_coord.1)
        .unwrap() = start_pipe;

    let x_max = maze_coords.iter().map(|(x, _)| *x).max().unwrap();
    let x_min = maze_coords.iter().map(|(x, _)| *x).min().unwrap();
    let y_max = maze_coords.iter().map(|(_, y)| *y).max().unwrap();
    let y_min = maze_coords.iter().map(|(_, y)| *y).min().unwrap();

    ((x_min + 1)..x_max)
        .flat_map(|x| ((y_min + 1)..y_max).map(move |y| (x, y)))
        .filter(|(x, y)| !maze_coords.contains(&(*x, *y)))
        .map(|(x, y)| {
            let left_pipes = maze_coords
                .iter()
                // get left pipes
                .filter(move |(i, j)| *i == x && *j < y)
                // convert to char
                .map(|(i, j)| *maze_map.get(*i).unwrap().get(*j).unwrap())
                // get only those significant
                .filter(|c| ['|', 'F', '7'].contains(c))
                .collect::<Vec<char>>();

            let bars = left_pipes.iter().filter(|c| **c == '|').count();
            let count_f = left_pipes.iter().filter(|c| **c == 'F').count();
            let count_7 = left_pipes.iter().filter(|c| **c == '7').count();

            // each vertical bar is a line
            // after F7 ~ LJ removed, then only F or 7 count which is larger,
            // use abs_diff as shortcut

            bars + count_f.abs_diff(count_7)
        })
        // odd left lines
        .filter(|left_lines| left_lines % 2 != 0)
        .count()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process_1() {
        let input = r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"#;
        assert_eq!(process(input), 10);
    }

    #[test]
    fn test_process_2() {
        let input = r#"..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
.........."#;
        assert_eq!(process(input), 4);
    }

    #[test]
    fn test_process_3() {
        let input = r#".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."#;
        assert_eq!(process(input), 8);
    }
}

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CoordType {
    Odd,
    Even,
}
impl std::ops::Neg for CoordType {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            CoordType::Odd => CoordType::Even,
            CoordType::Even => CoordType::Odd,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}
impl Coord {
    fn next_coords(&self, puzzlemap: &PuzzleMap) -> Vec<Coord> {
        let mut ret = vec![];
        if self.x == 0 {
            ret.push(Coord {
                x: self.x + 1,
                y: self.y,
            });
        } else if self.x == puzzlemap.max_x {
            ret.push(Coord {
                x: self.x - 1,
                y: self.y,
            });
        } else {
            ret.push(Coord {
                x: self.x + 1,
                y: self.y,
            });
            ret.push(Coord {
                x: self.x - 1,
                y: self.y,
            });
        }
        if self.y == 0 {
            ret.push(Coord {
                x: self.x,
                y: self.y + 1,
            });
        } else if self.y == puzzlemap.max_y {
            ret.push(Coord {
                x: self.x,
                y: self.y - 1,
            });
        } else {
            ret.push(Coord {
                x: self.x,
                y: self.y + 1,
            });
            ret.push(Coord {
                x: self.x,
                y: self.y - 1,
            });
        }
        ret.into_iter()
            .filter(|coord| puzzlemap.get(coord) != '#')
            .filter(|coord| !puzzlemap.cache.contains_key(coord))
            .collect()
    }
}

#[derive(Debug)]
struct PuzzleMap<'a> {
    input: &'a str,
    next_coords: Vec<Coord>,
    current_coord_type: CoordType,
    cache: HashMap<Coord, CoordType>,
    max_x: usize,
    max_y: usize,
}
impl<'a> PuzzleMap<'a> {
    fn new(input: &'a str) -> Self {
        let mut start = None;
        'outer: for (x, line) in input.lines().enumerate() {
            for (y, c) in line.chars().enumerate() {
                if c == 'S' {
                    start = Some(Coord { x, y });
                    break 'outer;
                }
            }
        }
        Self {
            input,
            next_coords: vec![start.clone().unwrap()],
            current_coord_type: CoordType::Even,
            cache: [(start.unwrap(), CoordType::Even)].into_iter().collect(),
            max_x: input.lines().count() - 1,
            max_y: input.lines().next().unwrap().len() - 1,
        }
    }
    fn get(&self, coord: &Coord) -> char {
        self.input
            .lines()
            .nth(coord.x)
            .unwrap()
            .chars()
            .nth(coord.y)
            .unwrap()
    }
    fn run(&mut self, steps: usize) -> usize {
        for _ in 0..steps {
            self.current_coord_type = -self.current_coord_type;
            let mut new_next_coords = vec![];
            while let Some(coord) = self.next_coords.pop() {
                let next_coords = coord.next_coords(self);
                for coord in next_coords {
                    self.cache.insert(coord.clone(), self.current_coord_type);
                    new_next_coords.push(coord);
                }
            }
            self.next_coords = new_next_coords;
        }
        if steps % 2 == 0 {
            self.cache
                .values()
                .filter(|coord_type| **coord_type == CoordType::Even)
                .count()
        } else {
            self.cache
                .values()
                .filter(|coord_type| **coord_type == CoordType::Odd)
                .count()
        }
    }
}
pub fn process(input: &str, steps: usize) -> usize {
    let mut puzzlemap = PuzzleMap::new(input);
    puzzlemap.run(steps)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."#;
        assert_eq!(process(input, 6), 16);
    }
}

pub mod part1;
pub mod part2;

enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
    Start,
}

#[derive(Debug)]
struct PipeParseError;

impl TryFrom<char> for Pipe {
    type Error = PipeParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Self::Vertical),
            '-' => Ok(Self::Horizontal),
            'J' => Ok(Self::NorthWest),
            'L' => Ok(Self::NorthEast),
            '7' => Ok(Self::SouthWest),
            'F' => Ok(Self::SouthEast),
            'S' => Ok(Self::Start),
            _other => Err(PipeParseError),
        }
    }
}

impl Pipe {
    fn connections(&self) -> [Direction; 2] {
        match self {
            Self::Vertical => [Direction::North, Direction::South],
            Self::Horizontal => [Direction::East, Direction::West],
            Self::NorthWest => [Direction::North, Direction::West],
            Self::NorthEast => [Direction::North, Direction::East],
            Self::SouthWest => [Direction::West, Direction::South],
            Self::SouthEast => [Direction::East, Direction::South],
            Self::Start => panic!("should not call connections() on start"),
        }
    }
}

type Coord = (usize, usize);
type Node = (Coord, Pipe);
struct PipeMaze<'a> {
    map: &'a str,
    start_pipe: Pipe,
    // stop_coord: Coord,
    cur: Node,
    nxt: Option<Node>,
}

impl<'a> PipeMaze<'a> {
    fn new(map: &'a str) -> Self {
        let rows_max = map.lines().count();
        let cols_max = map.lines().next().unwrap().len();

        let mut opt_cur_coord: Option<Coord> = None;
        let cur_pipe: Pipe;
        let nxt: Node;

        let mut up: Option<Coord> = None;
        let mut down: Option<Coord> = None;
        let mut left: Option<Coord> = None;
        let mut right: Option<Coord> = None;

        'check_start: for i in 0..rows_max {
            for j in 0..cols_max {
                let node = map.lines().nth(i).unwrap().chars().nth(j).unwrap();
                if node == 'S' {
                    opt_cur_coord = Some((i, j));
                    if i == 0 {
                        down = Some((i + 1, j));
                    } else if i == rows_max - 1 {
                        up = Some((i - 1, j));
                    } else {
                        up = Some((i - 1, j));
                        down = Some((i + 1, j));
                    }
                    if j == 0 {
                        right = Some((i, j + 1));
                    } else if j == cols_max - 1 {
                        left = Some((i, j - 1));
                    } else {
                        left = Some((i, j - 1));
                        right = Some((i, j + 1));
                    }
                    break 'check_start;
                }
            }
        }
        assert!(opt_cur_coord.is_some());
        let cur_coord = opt_cur_coord.unwrap();

        let pipe_up = up.and_then(|x| {
            (Pipe::try_from(map.lines().nth(x.0).unwrap().chars().nth(x.1).unwrap())).ok()
        });
        let pipe_down = down.and_then(|x| {
            (Pipe::try_from(map.lines().nth(x.0).unwrap().chars().nth(x.1).unwrap())).ok()
        });
        let pipe_left = left.and_then(|x| {
            (Pipe::try_from(map.lines().nth(x.0).unwrap().chars().nth(x.1).unwrap())).ok()
        });
        let pipe_right = right.and_then(|x| {
            (Pipe::try_from(map.lines().nth(x.0).unwrap().chars().nth(x.1).unwrap())).ok()
        });

        match (pipe_up, pipe_down, pipe_left, pipe_right) {
            (Some(u), Some(d), _, _)
                if [Pipe::Vertical, Pipe::SouthWest, Pipe::SouthEast].contains(&u)
                    && [Pipe::Vertical, Pipe::NorthWest, Pipe::NorthEast].contains(&d) =>
            {
                cur_pipe = Pipe::Vertical;
                nxt = (down.unwrap(), d);
            }
            (Some(u), _, Some(l), _)
                if [Pipe::Vertical, Pipe::SouthWest, Pipe::SouthEast].contains(&u)
                    && [Pipe::Horizontal, Pipe::SouthEast, Pipe::NorthEast].contains(&l) =>
            {
                cur_pipe = Pipe::NorthWest;
                nxt = (left.unwrap(), l);
            }
            (Some(u), _, _, Some(r))
                if [Pipe::Vertical, Pipe::SouthWest, Pipe::SouthEast].contains(&u)
                    && [Pipe::Horizontal, Pipe::SouthWest, Pipe::NorthWest].contains(&r) =>
            {
                cur_pipe = Pipe::NorthEast;
                nxt = (right.unwrap(), r);
            }
            (_, Some(d), Some(l), _)
                if [Pipe::Vertical, Pipe::NorthWest, Pipe::NorthEast].contains(&d)
                    && [Pipe::Horizontal, Pipe::SouthEast, Pipe::NorthEast].contains(&l) =>
            {
                cur_pipe = Pipe::SouthWest;
                nxt = (left.unwrap(), l);
            }
            (_, Some(d), _, Some(r))
                if [Pipe::Vertical, Pipe::NorthWest, Pipe::NorthEast].contains(&d)
                    && [Pipe::Horizontal, Pipe::SouthWest, Pipe::NorthWest].contains(&r) =>
            {
                cur_pipe = Pipe::SouthEast;
                nxt = (right.unwrap(), r);
            }
            (_, _, Some(l), Some(r))
                if [Pipe::Horizontal, Pipe::NorthEast, Pipe::SouthEast].contains(&l)
                    && [Pipe::Horizontal, Pipe::SouthWest, Pipe::NorthWest].contains(&r) =>
            {
                cur_pipe = Pipe::Horizontal;
                nxt = (right.unwrap(), r);
            }
            _ => unreachable!("Start doesnt have two connections"),
        };

        PipeMaze {
            map,
            start_pipe: cur_pipe,
            // stop_coord: nxt.0,
            cur: (cur_coord, cur_pipe),
            nxt: Some(nxt),
        }
    }
}

impl<'a> Iterator for PipeMaze<'a> {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        let curr_node = self.cur.clone();
        if let Some(nxt_node) = self.nxt {
            if nxt_node.1 == Pipe::Start {
                self.cur = self.nxt.unwrap();
                self.nxt = None;
                Some(curr_node)
            } else {
                let nxt_coord = nxt_node.0;
                let nxt_pipe = nxt_node.1;
                let new_nxt_coord = nxt_pipe
                    .connections()
                    .iter()
                    .find_map(|direction| {
                        let new_coord = match direction {
                            Direction::North => (nxt_coord.0 - 1, nxt_coord.1),
                            Direction::South => (nxt_coord.0 + 1, nxt_coord.1),
                            Direction::East => (nxt_coord.0, nxt_coord.1 + 1),
                            Direction::West => (nxt_coord.0, nxt_coord.1 - 1),
                        };
                        if new_coord != self.cur.0 {
                            Some(new_coord)
                        } else {
                            None
                        }
                    })
                    .unwrap();
                let new_nxt_pipe = Pipe::try_from(
                    self.map
                        .lines()
                        .nth(new_nxt_coord.0)
                        .unwrap()
                        .chars()
                        .nth(new_nxt_coord.1)
                        .unwrap(),
                )
                .unwrap();
                self.cur = self.nxt.unwrap();
                self.nxt = Some((new_nxt_coord, new_nxt_pipe));
                Some(curr_node)
            }
        } else {
            None
        }
    }
}

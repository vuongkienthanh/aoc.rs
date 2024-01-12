pub mod part1;
pub mod part2;
use std::collections::HashMap;
use std::collections::HashSet;

type Coord = (usize, usize, usize);

#[derive(Debug, Clone)]
struct Brick {
    name: usize,
    start: Coord,
    end: Coord,
}
impl Brick {
    fn to_coords(&self) -> Vec<Coord> {
        if self.start.0 != self.end.0 {
            ((self.start.0)..=(self.end.0))
                .map(|x| (x, self.start.1, self.start.2))
                .collect()
        } else if self.start.1 != self.end.1 {
            ((self.start.1)..=(self.end.1))
                .map(|y| (self.start.0, y, self.start.2))
                .collect()
        } else {
            ((self.start.2)..=(self.end.2))
                .map(|z| (self.start.0, self.start.1, z))
                .collect()
        }
    }
    fn downward(&self) -> Option<Self> {
        if self.start.2 == 1 {
            None
        } else {
            Some(Brick {
                name: self.name.clone(),
                start: (self.start.0, self.start.1, self.start.2 - 1),
                end: (self.end.0, self.end.1, self.end.2 - 1),
            })
        }
    }
}

#[derive(Debug, Clone)]
struct Tower {
    occupied: HashMap<Coord, usize>,
    support: HashMap<usize, Vec<usize>>, // what support whats
}

impl Tower {
    fn new(bricks: Vec<Brick>) -> Self {
        let mut tower = Self {
            occupied: HashMap::new(),
            support: HashMap::new(),
        };
        for brick in bricks {
            tower.add_brick(brick);
        }

        tower
    }
    fn add_brick(&mut self, brick: Brick) {
        self.support.insert(brick.name, vec![]);
        let mut last_brick = brick;
        let brick_dependencies: HashSet<usize>;
        loop {
            match last_brick.downward() {
                Some(new_brick) => {
                    let new_brick_coords = new_brick.to_coords();
                    if new_brick_coords
                        .iter()
                        .all(|c| !self.occupied.contains_key(&c))
                    {
                        last_brick = new_brick;
                    } else {
                        brick_dependencies = new_brick_coords
                            .into_iter()
                            .filter(|coord| self.occupied.contains_key(&coord))
                            .map(|coord| self.occupied.get(&coord).cloned().unwrap())
                            .collect();
                        break;
                    }
                }
                None => {
                    brick_dependencies = HashSet::new();
                    break;
                }
            }
        }
        for coord in last_brick.to_coords() {
            self.occupied.insert(coord, last_brick.name);
        }
        for brick in brick_dependencies {
            self.support.get_mut(&brick).unwrap().push(last_brick.name);
        }
    }
}

fn parse_input(input: &str) -> Vec<Brick> {
    let mut ret: Vec<Brick> = input
        .lines()
        .enumerate()
        .map(|(name, line)| {
            let (start_c, end_c) = line.split_once("~").unwrap();
            let mut start_i = start_c.split(',').map(|c| c.parse::<usize>().unwrap());
            let start = (
                start_i.next().unwrap(),
                start_i.next().unwrap(),
                start_i.next().unwrap(),
            );

            let mut end_i = end_c.split(',').map(|c| c.parse::<usize>().unwrap());
            let end = (
                end_i.next().unwrap(),
                end_i.next().unwrap(),
                end_i.next().unwrap(),
            );
            Brick { name, start, end }
        })
        .collect();
    ret.sort_unstable_by(|a, b| a.start.2.cmp(&b.start.2));
    ret
}

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct State {
    pub locations: [u8; 8],
    pub last_moved: usize,
    pub score: usize,
}

impl State {
    pub const SCORE: [usize; 8] = [1, 1, 10, 10, 100, 100, 1000, 1000];

    /// used for hashing and is_done
    pub fn normalized_locations(&self) -> [u8; 8] {
        let mut locations = self.locations;
        if locations[1] < locations[0] {
            locations.swap(0, 1)
        }
        if locations[3] < locations[2] {
            locations.swap(2, 3)
        }
        if locations[5] < locations[4] {
            locations.swap(4, 5)
        }
        if locations[7] < locations[6] {
            locations.swap(6, 7)
        }

        locations
    }

    pub fn is_done(&self) -> bool {
        self.normalized_locations() == [0, 1, 2, 3, 4, 5, 6, 7]
    }

    pub fn next_states(&self) -> Vec<State> {
        // check if any amphipod is already in right places
        let mut starters = vec![];
        match &self.locations[0..2] {
            [0, 1] | [1, 0] => (),
            [1, _] => starters.push(0),
            [0, _] => starters.push(1),
            _ => starters.extend([0, 1]),
        };
        match &self.locations[2..4] {
            [2, 3] | [3, 2] => (),
            [3, _] => starters.push(2),
            [2, _] => starters.push(3),
            _ => starters.extend([2, 3]),
        };
        match &self.locations[4..6] {
            [4, 5] | [5, 4] => (),
            [5, _] => starters.push(4),
            [4, _] => starters.push(5),
            _ => starters.extend([4, 5]),
        };
        match &self.locations[6..8] {
            [6, 7] | [7, 6] => (),
            [7, _] => starters.push(6),
            [6, _] => starters.push(7),
            _ => starters.extend([6, 7]),
        };

        let mut next = vec![];

        for amphipod in starters {
            // exclude last moved amphipod
            if amphipod == self.last_moved {
                continue;
            }
            // amphipod should only move to the hallway or their target room
            let ignored_rooms = match amphipod {
                0 | 1 => [2, 3, 4, 5, 6, 7],
                2 | 3 => [0, 1, 4, 5, 6, 7],
                4 | 5 => [0, 1, 2, 3, 6, 7],
                6 | 7 => [0, 1, 2, 3, 4, 5],
                _ => panic!(),
            };
            let from = self.locations[amphipod];
            for (step, pos) in available_paths(from) {
                // check obstacle
                if self.locations.iter().any(|lc| pos.contains(lc)) {
                    continue;
                }
                let to = pos.last().cloned().unwrap();
                // amphipod should only move to the hallway or their target room
                // so ignore other rooms
                if ignored_rooms.contains(&to) {
                    continue;
                }
                if [1, 3, 5, 7].contains(&to) {
                    // move to inner room if possible
                    if self.locations.iter().all(|lc| *lc != to - 1) {
                        continue;
                    }
                    // when moving to outer room, check if same type is presented
                    if self.locations[same_type(amphipod)] != to - 1 {
                        continue;
                    }
                }

                let mut locations = self.locations;
                locations[amphipod] = to;
                let score = self.score + step * State::SCORE[amphipod];
                next.push(State {
                    locations,
                    last_moved: amphipod,
                    score,
                });
            }
        }

        next
    }
}

impl From<&str> for State {
    fn from(value: &str) -> State {
        fn loc_to_id((r, c): (usize, usize)) -> u8 {
            match (r, c) {
                (3, 3) => 0,
                (2, 3) => 1,
                (3, 5) => 2,
                (2, 5) => 3,
                (3, 7) => 4,
                (2, 7) => 5,
                (3, 9) => 6,
                (2, 9) => 7,
                _ => panic!(),
            }
        }
        let input = value
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let (mut a2, mut b2, mut c2, mut d2) = (false, false, false, false);
        let mut locations = [0; 8];
        for (r, c) in [
            (2, 3),
            (2, 5),
            (2, 7),
            (2, 9),
            (3, 3),
            (3, 5),
            (3, 7),
            (3, 9),
        ] {
            match input[r][c] {
                'A' if !a2 => {
                    locations[0] = loc_to_id((r, c));
                    a2 = true;
                }
                'A' if a2 => locations[1] = loc_to_id((r, c)),
                'B' if !b2 => {
                    locations[2] = loc_to_id((r, c));
                    b2 = true;
                }
                'B' if b2 => locations[3] = loc_to_id((r, c)),
                'C' if !c2 => {
                    locations[4] = loc_to_id((r, c));
                    c2 = true;
                }
                'C' if c2 => locations[5] = loc_to_id((r, c)),
                'D' if !d2 => {
                    locations[6] = loc_to_id((r, c));
                    d2 = true;
                }
                'D' if d2 => locations[7] = loc_to_id((r, c)),
                _ => panic!(),
            }
        }
        State {
            locations,
            last_moved: 9,
            score: 0,
        }
    }
}

// ####1#1#1#11#
// #89.0.1.2.34#
// ###1#3#5#7###
//   #0#2#4#6#
//   #########

pub fn available_paths(from: u8) -> Vec<(usize, Vec<u8>)> {
    match from {
        0 => vec![
            (6, vec![1, 10, 3, 2]),
            (5, vec![1, 10, 3]),
            (8, vec![1, 10, 11, 5, 4]),
            (7, vec![1, 10, 11, 5]),
            (10, vec![1, 10, 11, 12, 7, 6]),
            (9, vec![1, 10, 11, 12, 7]),
            (4, vec![1, 9, 8]),
            (3, vec![1, 9]),
            (3, vec![1, 10]),
            (5, vec![1, 10, 11]),
            (7, vec![1, 10, 11, 12]),
            (9, vec![1, 10, 11, 12, 13]),
            (10, vec![1, 10, 11, 12, 13, 14]),
        ],
        1 => vec![
            (5, vec![10, 3, 2]),
            (4, vec![10, 3]),
            (7, vec![10, 11, 5, 4]),
            (6, vec![10, 11, 5]),
            (9, vec![10, 11, 12, 7, 6]),
            (8, vec![10, 11, 12, 7]),
            (3, vec![9, 8]),
            (2, vec![9]),
            (2, vec![10]),
            (4, vec![10, 11]),
            (6, vec![10, 11, 12]),
            (8, vec![10, 11, 12, 13]),
            (9, vec![10, 11, 12, 13, 14]),
        ],
        2 => vec![
            (6, vec![3, 10, 1, 0]),
            (5, vec![3, 10, 1]),
            (6, vec![3, 11, 5, 4]),
            (5, vec![3, 11, 5]),
            (8, vec![3, 11, 12, 7, 6]),
            (7, vec![3, 11, 12, 7]),
            (6, vec![3, 10, 9, 8]),
            (5, vec![3, 10, 9]),
            (3, vec![3, 10]),
            (3, vec![3, 11]),
            (5, vec![3, 11, 12]),
            (7, vec![3, 11, 12, 13]),
            (8, vec![3, 11, 12, 13, 13]),
        ],
        3 => vec![
            (5, vec![10, 1, 0]),
            (4, vec![10, 1]),
            (5, vec![11, 5, 4]),
            (4, vec![11, 5]),
            (7, vec![11, 12, 7, 6]),
            (6, vec![11, 12, 7]),
            (5, vec![10, 9, 8]),
            (4, vec![10, 9]),
            (2, vec![10]),
            (2, vec![11]),
            (4, vec![11, 12]),
            (6, vec![11, 12, 13]),
            (7, vec![11, 12, 13, 14]),
        ],
        4 => vec![
            (8, vec![5, 11, 10, 1, 0]),
            (7, vec![5, 11, 10, 1]),
            (6, vec![5, 11, 3, 2]),
            (5, vec![5, 11, 3]),
            (6, vec![5, 12, 7, 6]),
            (5, vec![5, 12, 7]),
            (8, vec![5, 11, 10, 9, 8]),
            (7, vec![5, 11, 10, 9]),
            (5, vec![5, 11, 10]),
            (3, vec![5, 11]),
            (3, vec![5, 12]),
            (5, vec![5, 12, 13]),
            (6, vec![5, 12, 13, 14]),
        ],
        5 => vec![
            (7, vec![11, 10, 1, 0]),
            (6, vec![11, 10, 1]),
            (5, vec![11, 3, 2]),
            (4, vec![11, 3]),
            (5, vec![12, 7, 6]),
            (4, vec![12, 7]),
            (7, vec![11, 10, 9, 8]),
            (6, vec![11, 10, 9]),
            (4, vec![11, 10]),
            (2, vec![11]),
            (2, vec![12]),
            (4, vec![12, 13]),
            (5, vec![12, 13, 14]),
        ],
        6 => vec![
            (10, vec![7, 12, 11, 10, 1, 0]),
            (9, vec![7, 12, 11, 10, 1]),
            (8, vec![7, 12, 11, 3, 2]),
            (7, vec![7, 12, 11, 3]),
            (6, vec![7, 12, 5, 4]),
            (5, vec![7, 12, 5]),
            (10, vec![7, 12, 11, 10, 9, 8]),
            (9, vec![7, 12, 11, 10, 9]),
            (7, vec![7, 12, 11, 10]),
            (5, vec![7, 12, 11]),
            (3, vec![7, 12]),
            (3, vec![7, 13]),
            (4, vec![7, 13, 14]),
        ],
        7 => vec![
            (9, vec![12, 11, 10, 1, 0]),
            (8, vec![12, 11, 10, 1]),
            (7, vec![12, 11, 3, 2]),
            (6, vec![12, 11, 3]),
            (5, vec![12, 5, 4]),
            (4, vec![12, 5]),
            (9, vec![12, 11, 10, 9, 8]),
            (8, vec![12, 11, 10, 9]),
            (6, vec![12, 11, 10]),
            (4, vec![12, 11]),
            (2, vec![12]),
            (2, vec![13]),
            (3, vec![14]),
        ],
        8 => vec![
            (4, vec![9, 1, 0]),
            (3, vec![9, 1]),
            (6, vec![9, 10, 3, 2]),
            (5, vec![9, 10, 3]),
            (8, vec![9, 10, 11, 5, 4]),
            (7, vec![9, 10, 11, 5]),
            (10, vec![9, 10, 11, 12, 7, 6]),
            (9, vec![9, 10, 11, 12, 7]),
        ],
        9 => vec![
            (3, vec![1, 0]),
            (2, vec![1]),
            (5, vec![10, 3, 2]),
            (4, vec![10, 3]),
            (7, vec![10, 11, 5, 4]),
            (6, vec![10, 11, 5]),
            (9, vec![10, 11, 12, 7, 6]),
            (8, vec![10, 11, 12, 7]),
        ],
        10 => vec![
            (3, vec![1, 0]),
            (2, vec![1]),
            (3, vec![3, 2]),
            (2, vec![3]),
            (5, vec![11, 5, 4]),
            (4, vec![11, 5]),
            (7, vec![11, 12, 7, 6]),
            (6, vec![11, 12, 7]),
        ],
        11 => vec![
            (5, vec![10, 1, 0]),
            (4, vec![10, 1]),
            (3, vec![3, 2]),
            (2, vec![3]),
            (3, vec![5, 4]),
            (2, vec![5]),
            (5, vec![12, 7, 6]),
            (4, vec![12, 7]),
        ],
        12 => vec![
            (7, vec![11, 10, 1, 0]),
            (6, vec![11, 10, 1]),
            (5, vec![11, 3, 2]),
            (4, vec![11, 3]),
            (3, vec![5, 4]),
            (2, vec![5]),
            (3, vec![7, 6]),
            (2, vec![7]),
        ],
        13 => vec![
            (9, vec![12, 11, 10, 1, 0]),
            (8, vec![12, 11, 10, 1]),
            (7, vec![12, 11, 3, 2]),
            (6, vec![12, 11, 3]),
            (5, vec![12, 5, 4]),
            (4, vec![12, 5]),
            (3, vec![7, 6]),
            (2, vec![7]),
        ],
        14 => vec![
            (10, vec![13, 12, 11, 10, 1, 0]),
            (9, vec![13, 12, 11, 10, 1]),
            (8, vec![13, 12, 11, 3, 2]),
            (7, vec![13, 12, 11, 3]),
            (6, vec![13, 12, 5, 4]),
            (5, vec![13, 12, 5]),
            (4, vec![13, 7, 6]),
            (3, vec![13, 7]),
        ],
        _ => panic!(),
    }
}

fn same_type(a: usize) -> usize {
    match a {
        0 => 1,
        1 => 0,
        2 => 3,
        3 => 2,
        4 => 5,
        5 => 4,
        6 => 7,
        7 => 6,
        _ => panic!(),
    }
}

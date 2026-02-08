use intcode::{Computer, parse};

pub fn process(_input: &str) -> usize {
    let input = parse(_input);
    let drone = Computer::new(input);

    let mut y = 100;
    while {
        let left = left_x(y, &drone);
        get((left + 99, y - 99), &drone) == 0
    } {
        y += 100;
    }
    let mut low = y - 100;
    let mut high = y;
    while low < high - 1 {
        let mid = (low + high) / 2;
        let left = left_x(mid, &drone);
        if get((left + 99, mid - 99), &drone) == 1 {
            high = mid;
        } else {
            low = mid
        }
    }
    left_x(high, &drone) * 10000 + high - 99
}

fn left_x(y: usize, drone: &Computer) -> usize {
    (y..)
        .find_map(|x| (get((x, y), drone) == 1).then_some(x))
        .unwrap()
}

fn get((x, y): (usize, usize), drone: &Computer) -> usize {
    let mut drone = drone.clone();
    drone.input(x as i64);
    drone.input(y as i64);
    drone.long_run().output() as usize
}

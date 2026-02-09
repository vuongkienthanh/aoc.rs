pub mod part1;
pub mod part2;

fn parse(input: &str) -> (usize, Vec<Option<usize>>) {
    let mut line = input.lines();
    let target = line.next().unwrap().parse::<usize>().unwrap();
    let buses = line
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<usize>().ok())
        .collect();

    (target, buses)
}

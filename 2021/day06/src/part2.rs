use crate::{get_cache, count};

pub fn process(_input: &str) -> usize {
    let mut cache = get_cache();
    _input
        .split(",")
        .map(|x| (x.parse::<usize>().unwrap(), 256))
        .map(|(fish, day)| count(fish, day, &mut cache))
        .sum()
}

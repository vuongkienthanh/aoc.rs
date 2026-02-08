use itertools::Itertools;

pub fn process(_input: &str) -> usize {
    let data = _input
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let sum: usize = data.iter().copied().sum();
    let quarter = sum / 4;

    for i in 2..=data.len() / 2 {
        for comb in data.iter().combinations(i) {
            if comb.iter().copied().sum::<usize>() == quarter {
                return comb.iter().copied().product();
            }
        }
    }
    panic!("should have an answer")
}

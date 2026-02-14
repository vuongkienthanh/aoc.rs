use itertools::Itertools;

pub fn process(_input: &str) -> usize {
    let input: Vec<_> = _input
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let (min, max) = input.iter().minmax().into_option().unwrap();
    let mut ans = usize::MAX;
    for loc in *min..=*max {
        let cost = input
            .iter()
            .map(|x| {
                let dst = x.abs_diff(loc);
                dst * (dst + 1) / 2
            })
            .sum();
        ans = ans.min(cost);
    }
    ans
}

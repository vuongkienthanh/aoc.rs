use itertools::Itertools;

pub fn process(_input: &str) -> usize {
    let input : Vec<_>= _input.split(",").map(|x| x.parse::<usize>().unwrap()).collect();
    let (min, max) = input.iter().minmax().into_option().unwrap();
    let mut ans = usize::MAX;
    for loc in *min..=*max {
        ans = ans.min(input.iter().map(|x| x.abs_diff(loc)).sum());
    }
    ans
}

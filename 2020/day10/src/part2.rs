use crate::parse;

pub fn process(_input: &str) -> usize {
    let input = parse(_input);
    let mut cache = vec![0; input.len()];
    *cache.last_mut().unwrap() = 1;
    way(0, &input, &mut cache)
}

fn way(i: usize, input: &[usize], cache: &mut [usize]) -> usize {
    if let Some(s) = cache.get(i)
        && s != &0
    {
        return *s;
    }
    let ans = (i + 1..input.len())
        .take_while(|j| input[*j] - input[i] <= 3)
        .map(|i| way(i, input, cache))
        .sum();
    cache[i] = ans;
    ans
}

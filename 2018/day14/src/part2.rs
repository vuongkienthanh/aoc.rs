use crate::part1::step;

pub fn process(_input: &str) -> usize {
    let (mut v, mut a, mut b) = (vec![3u8, 7], 0 , 1);
    let target :Vec<_> = _input.chars().map(|x| x.to_digit(10).unwrap() as u8).collect();
    let len = _input.len();

    loop {
        step(&mut v, &mut a, &mut b);
        if &v[v.len().saturating_sub(len)..v.len()] == &target {
            break v.len() - len;
        }
        if &v[(v.len() -1).saturating_sub(len)..(v.len()-1)] == &target {
            break v.len() - len -1;
        }
    }
}

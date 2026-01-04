pub fn process(_input: &str) -> String {
    let mut v = vec![3u8, 7];
    let (mut a, mut b) = (0, 1);
    
    let input = _input.parse::<usize>().unwrap();
    
    for _ in 0..input + 10 {
        step(&mut v, &mut a, &mut b);
    }
    v.into_iter().skip(input).take(10).map(|x| char::from_digit(x as u32, 10).unwrap()).collect::<String>()
}

fn step(v: &mut Vec<u8>, a: &mut usize, b: &mut usize) {
    // println!("{v:?}");
    let n = v[*a] + v[*b];
    if n >= 10 {
        v.push(1);
        v.push(n%10);
    } else {
        v.push(n);
    }
    *a = (*a + 1 + v[*a] as usize) % v.len();
    *b = (*b + 1 + v[*b] as usize) % v.len();
}
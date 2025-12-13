pub fn gcd(mut a: isize, mut b: isize) -> isize {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a.abs()
}

pub fn lcm(a: isize, b: isize) -> isize {
    if a == 0 || b == 0 {
        0
    } else {
        (a / gcd(a, b)) * b
    }
    .abs()
}

pub fn gcd_vec(nums: &[isize]) -> isize {
    nums.iter().copied().reduce(gcd).unwrap()
}

pub fn lcm_vec(nums: &[isize]) -> isize {
    nums.iter().copied().reduce(lcm).unwrap()
}


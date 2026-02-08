use num_traits::{PrimInt, Zero};

pub fn gcd<T>(mut a: T, mut b: T) -> T
where
    T: PrimInt + Zero,
{
    while b != Zero::zero() {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

pub fn lcm<T>(a: T, b: T) -> T
where
    T: PrimInt + Zero,
{
    if a == Zero::zero() || b == Zero::zero() {
        Zero::zero()
    } else {
        (a / gcd(a, b)) * b
    }
}

pub fn gcd_vec<T>(nums: &[T]) -> T
where
    T: PrimInt + Zero,
{
    nums.iter().copied().reduce(gcd).unwrap()
}

pub fn lcm_vec<T>(nums: &[T]) -> T
where
    T: PrimInt + Zero,
{
    nums.iter().copied().reduce(lcm).unwrap()
}

pub mod chinese_remainder_theorem;

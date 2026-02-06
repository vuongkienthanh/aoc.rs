use crate::parsing::{Item, parse_input};
use aoc_helper::nom::Sign;

pub fn process(_input: &str) -> u128 {
    let input = parse_input(_input);
    let ncards = 119315717514047u128;
    let c = 2020;
    let M = 101741582076661u128;
    let (a, b) = combine_input(input, ncards);
    let Ma = mod_pow(a, M, ncards);
    let Mb = ((b * (Ma - 1) % ncards) * inv(a - 1, ncards)) % ncards;

    ((c + ncards - Mb) * inv(Ma, ncards)) % ncards
}

// deal into new stack: new_pos = -current_pos + deck_size - 1
// cut: new_pos = current_pos - cut_pos
// deal with increment: new_pos = increment * current_pos
// combine with algebra
// f = a*x+b and g = c*x+d, composition g(f(x)) is c*a*x + c*b + d.
fn combine_input(input: Vec<Item>, ncards: u128) -> (u128, u128) {
    let (mut a, mut b) = (1isize, 0isize);
    let ncards = ncards as isize;
    for item in input {
        let (c, d) = match item {
            Item::Reverse => (-1, -1),
            Item::Deal(x) => (x as isize, 0),
            Item::Cut(s, u) => match s {
                Sign::Positive => (1, -(u as isize)),
                Sign::Negative => (1, u as isize),
            },
        };
        a = (c * a + ncards) % ncards;
        b = (c * b + d + ncards) % ncards;
    }
    (a as u128, b as u128)
}

fn inv(a: u128, ncards: u128) -> u128 {
    mod_pow(a, ncards - 2, ncards)
}
fn mod_pow(base: u128, exp: u128, modulus: u128) -> u128 {
    let mut base = base as u128;
    let mut exp = exp as u128;
    let modulus = modulus as u128;
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            // Using modular multiplication at each step prevents overflow
            result = (result * base) % modulus;
        }
        exp >>= 1; // equivalent to exp / 2
        base = (base * base) % modulus;
    }
    result
}

use crate::parsing::parse_input;
use crate::{ORDER1, ORDER2};

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    input
        .into_iter()
        .filter(|group| {
            let keys = group
                .into_iter()
                .map(|(k, _)| k)
                .cloned()
                .collect::<Vec<_>>();
            if keys == ORDER1 {
                is_valid(
                    group[0].1, group[2].1, group[3].1, group[4].1, group[5].1, group[6].1,
                    group[7].1,
                )
            } else if keys == ORDER2 {
                is_valid(
                    group[0].1, group[1].1, group[2].1, group[3].1, group[4].1, group[5].1,
                    group[6].1,
                )
            } else {
                false
            }
        })
        .count()
}

fn is_valid(byr: &str, ecl: &str, eyr: &str, hcl: &str, hgt: &str, iyr: &str, pid: &str) -> bool {
    is_byr(byr)
        && is_ecl(ecl)
        && is_eyr(eyr)
        && is_hcl(hcl)
        && is_hgt(hgt)
        && is_iyr(iyr)
        && is_pid(pid)
}

fn is_byr(input: &str) -> bool {
    input.parse::<usize>().is_ok_and(|x| x >= 1920 && x <= 2002)
}
fn is_ecl(input: &str) -> bool {
    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&input)
}
fn is_eyr(input: &str) -> bool {
    input.parse::<usize>().is_ok_and(|x| x >= 2020 && x <= 2030)
}
fn is_hcl(input: &str) -> bool {
    input.chars().next().unwrap() == '#'
        && input.len() == 7
        && input[1..]
            .bytes()
            .all(|c| (b'0'..=b'9').contains(&c) || (b'a'..=b'z').contains(&c))
}
fn is_hgt(input: &str) -> bool {
    let l = input.len();
    (&input[l - 2..l] == "cm"
        && input[0..l - 2]
            .parse::<usize>()
            .is_ok_and(|x| x >= 150 && x <= 193))
        || (&input[l - 2..l] == "in"
            && input[0..l - 2]
                .parse::<usize>()
                .is_ok_and(|x| x >= 59 && x <= 76))
}
fn is_iyr(input: &str) -> bool {
    input.parse::<usize>().is_ok_and(|x| x >= 2010 && x <= 2020)
}
fn is_pid(input: &str) -> bool {
    input.len() == 9 && input.parse::<usize>().is_ok()
}

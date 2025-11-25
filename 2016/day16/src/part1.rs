use crate::{dragon_curve, checksum};

pub fn process(_input: &str) -> String {
    let a = _input.to_string();
    let disk = 272;
    checksum(dragon_curve(a, disk))
}

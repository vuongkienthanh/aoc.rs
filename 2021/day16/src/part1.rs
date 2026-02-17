use crate::parsing::{Packet, parse_input};

pub fn process(_input: &str) -> u32 {
    let input = parse_input(_input);
    score(input)
}

fn score(input: Packet) -> u32 {
    input.version + input.subpackets.into_iter().map(|p| score(p)).sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("8A004A801A8002F478", 16)]
    #[case("620080001611562C8802118E34", 12)]
    #[case("C0015000016115A2E0802F182340", 23)]
    #[case("A0016C880162017C3686B18A3D4780", 31)]
    fn test_process(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(process(input), expected);
    }
}

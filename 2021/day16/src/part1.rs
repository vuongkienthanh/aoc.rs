pub enum Waiting {
    Version,
    TypeId,
    LengthTypeId,
    Bit15,
    SubPacket11,
    Literal,
    Done,
}
pub fn process(_input: &str) -> u32 {
    let mut input = _input.chars().map(|x| match x {
        '0' => 0b0000,
        '1' => 0b0001,
        '2' => 0b0010,
        '3' => 0b0011,
        '4' => 0b0100,
        '5' => 0b0101,
        '6' => 0b0110,
        '7' => 0b0111,
        '8' => 0b1000,
        '9' => 0b1001,
        'A' => 0b1010,
        'B' => 0b1011,
        'C' => 0b1100,
        'D' => 0b1101,
        'E' => 0b1110,
        'F' => 0b1111,
        _ => panic!(),
    });
    let mut buffer: u32 = 0;
    let mut len = 0;
    let mut ans = 0;
    let mut status = Waiting::Version;
    let mut version = 0;
    let mut typeid = 0;
    let mut literal = 0;
    let mut lengthtypeid = 0;
    let mut bit15 = 0;
    let mut subpacket11 = 0;
    loop {
        while len <= 16 {
            if let Some(n) = input.next() {
                buffer = buffer << 4 | n;
                len += 4;
            } else {
                break;
            }
        }
        match status {
            Waiting::Version => {
                len -= 3;
                if bit15 > 0 {
                    bit15 -= 3;
                }
                if subpacket11 > 0 {
                    subpacket11 -= 1;
                }
                version = buffer >> len;
                ans += version;
                buffer ^= version << len;
                status = Waiting::TypeId;
                println!("version = {version}");
            }
            Waiting::TypeId => {
                len -= 3;
                if bit15 > 0 {
                    bit15 -= 3;
                }
                typeid = buffer >> len;
                buffer ^= typeid << len;
                println!("typeid = {typeid}");
                if typeid == 4 {
                    status = Waiting::Literal;
                } else {
                    status = Waiting::LengthTypeId;
                }
            }
            Waiting::Literal => {
                len -= 5;
                if bit15 > 0 {
                    bit15 -= 5;
                }
                let group = buffer >> len;
                buffer ^= group << len;
                if group >> 4 == 1 {
                    literal = literal << 4 | (group ^ 0b10000);
                } else {
                    literal = literal << 4 | group;
                    status = Waiting::Done;
                    println!("literal ={}", literal);
                }
            }
            Waiting::LengthTypeId => {
                len -= 1;
                if bit15 > 0 {
                    bit15 -= 1;
                }
                lengthtypeid = buffer >> len;
                buffer ^= lengthtypeid << len;
                println!("lengthtypeid = {lengthtypeid}");
                if lengthtypeid == 0 {
                    status = Waiting::Bit15;
                } else {
                    status = Waiting::SubPacket11;
                }
            }
            Waiting::Bit15 => {
                len -= 15;
                if bit15 > 0 {
                    bit15 -= 15;
                }
                let this_bit15 = buffer >> len;
                buffer ^= this_bit15 << len;
                status = Waiting::Version;
                bit15 += this_bit15;
                println!("bit15 = {bit15}");
            }
            Waiting::SubPacket11 => {
                len -= 11;
                if bit15 > 0 {
                    bit15 -= 11;
                }
                let this_subpacket11 = buffer >> len;
                println!("this_subpacket11 = {this_subpacket11}");
                buffer ^= this_subpacket11 << len;
                status = Waiting::Version;
                subpacket11 += this_subpacket11;
                println!("subpacket11 = {subpacket11}");
            }
            Waiting::Done => {
                literal = 0;
                println!("done with subpacket11 {subpacket11} bit15 {bit15}");
                if subpacket11 > 0 {
                    status = Waiting::Version;
                } else if bit15 > 0 {
                    status = Waiting::Version;
                } else {
                    break;
                }
            }
        }
    }

    ans
}

struct RunResult {
    version: u32,
    typeid: u32,
    lengthtypeid: u32,
    subpackets: Vec<RunResult>,
    bit_used:u32,
}

fn parse_packet(i: &mut impl<Item=u32>) -> RunResult{
    todo!()
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

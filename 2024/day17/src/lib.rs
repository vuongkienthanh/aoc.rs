pub mod part1;
pub mod part2;

struct Computer {
    register_a: usize,
    register_b: usize,
    register_c: usize,
    pointer: usize,
    output: Vec<usize>,
}

struct Instruction;

impl Instruction {
    fn run(opcode: char, value: usize, computer: &mut Computer) {
        match opcode {
            '0' => Self::adv(value, computer),
            '1' => Self::bxl(value, computer),
            '2' => Self::bst(value, computer),
            '3' => Self::jnz(value, computer),
            '4' => Self::bxc(value, computer),
            '5' => Self::out(value, computer),
            '6' => Self::bdv(value, computer),
            '7' => Self::cdv(value, computer),
            _ => unreachable!(),
        }
    }
    fn combo(value: usize, computer: &Computer) -> usize {
        match value {
            0..=3 => value,
            4 => computer.register_a,
            5 => computer.register_b,
            6 => computer.register_c,
            _ => unreachable!(),
        }
    }
    fn adv(value: usize, computer: &mut Computer) {
        println!("adv");
        let numerator = computer.register_a;
        let denominator = 2usize.pow(Self::combo(value, computer) as u32);
        let res = numerator / denominator;
        computer.register_a = res;
        computer.pointer += 2;
    }
    fn bxl(value: usize, computer: &mut Computer) {
        println!("bxl");
        let res = computer.register_b ^ value;
        computer.register_b = res;
        computer.pointer += 2;
    }
    fn bst(value: usize, computer: &mut Computer) {
        println!("bst");
        let res = Self::combo(value, computer) % 8;
        computer.register_b = res;
        computer.pointer += 2;
    }
    fn jnz(value: usize, computer: &mut Computer) {
        println!("jnz");
        if computer.register_a != 0 {
            computer.pointer = value;
        }
    }
    fn bxc(value: usize, computer: &mut Computer) {
        println!("bxc");
        let res = computer.register_b ^ computer.register_c;
        computer.register_b = res;
        computer.pointer += 2;
    }
    fn out(value: usize, computer: &mut Computer) {
        println!("out");
        let res = Self::combo(value, computer) % 8;
        computer.output.push(res);
        computer.pointer += 2;
    }
    fn bdv(value: usize, computer: &mut Computer) {
        println!("bdv");
        let numerator = computer.register_a;
        let denominator = 2usize.pow(Self::combo(value, computer) as u32);
        let res = numerator / denominator;
        computer.register_b = res;
        computer.pointer += 2;
    }
    fn cdv(value: usize, computer: &mut Computer) {
        println!("cdv");
        let numerator = computer.register_a;
        let denominator = 2usize.pow(Self::combo(value, computer) as u32);
        let res = numerator / denominator;
        computer.register_c = res;
        computer.pointer += 2;
    }
}

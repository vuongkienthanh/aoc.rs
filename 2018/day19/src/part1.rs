use crate::Device;
use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let (p, cmds) = parse_input(_input);
    let mut device = Device::new(p);

    let mut _i = 0;

    loop {
        if device.run_cmds(&cmds) {
            break;
        }
        // println!("{_i} : {:?}", device.registers);
        // println!("{:?}", cmds[device.get_pointer()]);
        // _i += 1;
        // if _i > 100 {
        //     break;
        // }
    }
    println!("ans = ");
    device.registers[0]
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 7);
    }
}

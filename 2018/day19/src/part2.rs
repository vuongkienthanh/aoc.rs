use crate::Device;
use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    // after studying the assembly
    // the problem is to find all divisors of the last register
    // in my input, it is:
    // part 1: 967
    // part 2: 10551367

    let (p, cmds) = parse_input(_input);
    let mut device = Device::new_with_a(p, 1);

    // the last register is found at i=17
    for _i in 0..17 {
        device.run_cmds(&cmds);
        // println!("{_i} : {:?}", device.registers);
        // println!("{:?}", cmds[device.get_pointer()]);
    }

    let target = device.registers.into_iter().last().unwrap();
    println!("target = {} ; it's sqrt is {}", target, target.isqrt());

    println!("first pair: 1 {target}");
    let mut ans = target + 1;

    println!("\ncheck sqrt:");
    if target.isqrt().pow(2u32) == target {
        println!("(sqrt^2 == target) => add its square => {}", target.isqrt());
        ans += target.isqrt();
    }

    println!("\nchecking from 2 to it's sqrt (exclusive)");
    for x in 2..target.isqrt() {
        if target % x == 0 {
            ans += x;
            ans += target / x;
            println!("found pair: {x} {}", target / x);
        }
    }

    println!("\nans = ");
    ans
}

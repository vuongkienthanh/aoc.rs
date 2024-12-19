pub fn process(_input: &str) -> usize {
    let prog = _input
        .split_once("\n\n")
        .unwrap()
        .1
        .split_once(": ")
        .unwrap()
        .1
        .chars()
        .step_by(2)
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();

    let mut va = vec![0];

    for num in prog.into_iter().rev() {
        let mut new_va = vec![];
        for a in va {
            for i in [0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b110, 0b111] {
                if find_b((a << 3) | i) == num {
                    new_va.push((a << 3) | i);
                }
            }
        }
        va = new_va;
    }

    for a in &va {
        println!("{a}");
        let recheck = super::part1::process(
            format!(
                r#"Register A: {}
Register B: 0
Register C: 0

Program: 2,4,1,1,7,5,4,0,0,3,1,6,5,5,3,0"#,
                a
            )
            .as_str(),
        );

        println!("Program: 2,4,1,1,7,5,4,0,0,3,1,6,5,5,3,0");
        println!("recheck: {recheck}");
    }
    va.into_iter().min().unwrap()
}

fn find_b(a: usize) -> usize {
    let mut b = a & 0b111;
    b ^= 1;
    b ^= a >> b;
    b ^= 6;
    b & 0b111
}

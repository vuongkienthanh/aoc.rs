use intcode::{Computer, RunResult, parse};

pub fn process(_input: &str) -> usize {
    let input = parse(_input);

    let mut computers = vec![];
    for i in 0..50 {
        let mut comp = Computer::new(input.clone());
        comp.input(i as i64);
        computers.push(comp);
    }

    'a: loop {
        for i in 0..50 {
            let comp = computers.get_mut(i).unwrap();
            let mut sent = None;
            match comp.long_run() {
                RunResult::Output(address) => {
                    let x = comp.long_run().output();
                    let y = comp.long_run().output();
                    if address == 255 {
                        break 'a y as usize;
                    }
                    sent = Some((address, x, y));
                }
                RunResult::WaitingInput => {
                    comp.input(-1);
                }
                RunResult::Halt => (),
            }
            if let Some((address, x, y)) = sent {
                let comp = computers.get_mut(address as usize).unwrap();
                comp.input(x);
                comp.input(y);
            }
        }
    }
}

use intcode::{Computer, RunResult, parse};

pub fn process(_input: &str) -> usize {
    let input = parse(_input);
    let mut NAT = (0, 0);
    let mut last_y = 0;

    let mut computers = vec![];
    for i in 0..50 {
        let mut comp = Computer::new(input.clone());
        comp.input(i as i64);
        computers.push(comp);
    }

    let mut idle = false;
    'a: loop {
        let mut this_time_idle = true;
        for i in 0..50 {
            let comp = computers.get_mut(i).unwrap();
            let mut sent = None;
            match comp.long_run() {
                RunResult::Output(address) => {
                    let x = comp.long_run().output();
                    let y = comp.long_run().output();
                    sent = Some((address, x, y));
                }
                RunResult::WaitingInput => {
                    comp.input(-1);
                }
                RunResult::Halt => panic!("should not halt"),
            }
            if let Some((address, x, y)) = sent {
                this_time_idle = false;
                if address == 255 {
                    NAT = (x, y);
                } else {
                    let comp = computers.get_mut(address as usize).unwrap();
                    comp.input(x);
                    comp.input(y);
                }
            }
        }
        if idle && this_time_idle {
            let comp = computers.get_mut(0).unwrap();
            comp.input(NAT.0);
            comp.input(NAT.1);
            if NAT.1 == last_y {
                break 'a last_y as usize;
            } else {
                last_y = NAT.1;
            }
        }
        idle = this_time_idle;
    }
}

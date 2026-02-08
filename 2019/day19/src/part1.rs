use intcode::{Computer, parse};

pub fn process(_input: &str) -> usize {
    let input = parse(_input);
    let drone = Computer::new(input);

    let mut ans = 0;
    for y in 0..50 {
        print!("row = {y}");
        for x in 0..50 {
            let mut drone = drone.clone();
            drone.input(x);
            drone.input(y);
            let o = drone.long_run().output() as usize;
            if o == 0 {
                print!(".");
            } else {
                print!("#");
            }
            ans += o;
        }
        println!("");
    }
    ans
}

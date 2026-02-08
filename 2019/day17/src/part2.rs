use crate::{
    build_main_function, build_map, build_move_function, divide_a_b_c, get_a_b_c, get_path,
};
use intcode::{Computer, RunResult, parse};

pub fn process(_input: &str) -> usize {
    let mut camera = Computer::new(parse(_input));
    let (map, robo) = build_map(camera.clone());
    let path = get_path(&map, robo);

    let paths = vec![path.clone()];
    let a = get_a_b_c(&paths);
    let paths = divide_a_b_c(paths, &a);
    let b = get_a_b_c(&paths);
    let paths = divide_a_b_c(paths, &b);
    let c = get_a_b_c(&paths);
    let paths = divide_a_b_c(paths, &c);
    assert!(paths.is_empty());

    let am = build_move_function(&a);
    let bm = build_move_function(&b);
    let cm = build_move_function(&c);
    let main_function = build_main_function(path, &a, &b, &c);
    println!("main_function = {main_function}");
    println!("A = {am}");
    println!("B = {bm}");
    println!("C = {cm}");

    let mut camera_input = [main_function, am, bm, cm].join("\n");
    camera_input.push_str("\nn\n");
    let mut camera_input_iterator = camera_input.bytes();
    camera.prog.insert(0, 2);
    let mut ans = 0;
    loop {
        match camera.long_run() {
            RunResult::Halt => break,
            RunResult::WaitingInput => camera.input(camera_input_iterator.next().unwrap() as i64),
            RunResult::Output(o) => ans = o,
        }
    }
    ans as usize
}

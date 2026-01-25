use crate::{
    build_main_function, build_map, build_move_function, divide_a, divide_b_c, get_a, get_b_c,
    get_path,
};
use intcode::{Computer, RunResult, parse};

pub fn process(_input: &str) -> usize {
    let (map, robo) = build_map(_input);
    let path = get_path(&map, robo);
    let a = get_a(&path);
    let rest = divide_a(path.clone(), &a);
    let b = get_b_c(&rest);
    let rest = divide_b_c(rest, &b);
    let c = get_b_c(&rest);
    let rest = divide_b_c(rest, &c);
    assert!(rest.is_empty());

    let am = build_move_function(&a);
    let bm = build_move_function(&b);
    let cm = build_move_function(&c);
    let main_function = build_main_function(path, &a, &b, &c);
    println!("main_function = {main_function}");
    println!("A = {am}");
    println!("B = {bm}");
    println!("C = {cm}");

    let mut camera_input = [main_function, am, bm, cm, "n".to_string()].join("\n");
    camera_input.push('\n');
    let mut camera_input_iterator = camera_input.chars();
    let mut camera = Computer::new(parse(_input));
    camera.prog.insert(0, 2);
    let mut ans = 0;
    loop {
        match camera.long_run() {
            RunResult::Halt => break,
            RunResult::WaitingInput => {
                camera.input(camera_input_iterator.next().unwrap() as u8 as i64)
            }
            RunResult::Output(o) => ans = o,
        }
    }
    ans as usize
}

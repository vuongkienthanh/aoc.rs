use intcode::{Computer, RunResult, parse};

pub fn process(_input: &str) -> usize {
    let mut comp = Computer::new(parse(_input));

    let springscript = r#"NOT C J 
AND D J 
NOT A T 
OR T J
WALK
"#;

    for c in springscript.bytes() {
        comp.input(c as i64);
    }
    let mut ans =0;
    while let RunResult::Output(o) = comp.long_run() {
        ans = o;
        print!("{}", o as u8 as char);
    }
    ans as usize
}

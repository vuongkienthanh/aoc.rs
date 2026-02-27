use day23::part1b::process;
fn main() {
    let input = include_str!("../input.txt").trim_end_matches("\r\n").trim_end_matches("\n");
    println!("{}",process(input));
}

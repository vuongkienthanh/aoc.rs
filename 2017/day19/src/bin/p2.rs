use day19::part2::process;
fn main() {
    let input = include_str!("../input.txt").trim_end_matches("\r\n");
    println!("{}", process(input));
}

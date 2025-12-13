use day24::part1::process;
fn main() {
    let input = include_str!("../input.txt");
    println!(
        "{}",
        process(
            input,
            200000000000000.0,
            400000000000000.0,
            200000000000000.0,
            400000000000000.0
        )
    );
}

pub fn process(_input: &str) -> usize {
    #[allow(non_snake_case)]
    let (mut NS, mut EW) = (0isize, 0isize);

    for d in _input.split(",") {
        match d {
            "n" => NS += 1,
            "s" => NS -= 1,
            "ne" => {
                NS += 1;
                EW += 1;
            }
            "nw" => {
                NS += 1;
                EW -= 1;
            }
            "se" => {
                NS -= 1;
                EW += 1;
            }
            "sw" => {
                NS -= 1;
                EW -= 1;
            }
            _ => panic!("unknown direction"),
        }
    }
    NS.unsigned_abs().max(EW.unsigned_abs())
}

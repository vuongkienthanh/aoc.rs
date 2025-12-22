pub fn process(_input: &str) -> usize {
    let target = _input.parse::<usize>().unwrap();
    let mut arr = vec![0; target / 11];

    for elf in 0..target / 11 {
        arr[elf] += (elf + 1) * 11;
        if arr[elf] >= target {
            return elf + 1;
        }
        for house in (elf..target / 11).step_by(elf + 1).skip(1).take(50) {
            arr[house] += (elf + 1) * 11;
        }
    }
    panic!("should have an answer")
}

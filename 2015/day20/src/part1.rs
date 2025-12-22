pub fn process(_input: &str) -> usize {
    let target = _input.parse::<usize>().unwrap();
    let mut arr = vec![0; target / 10];

    for elf in 0..target / 10 {
        arr[elf] += (elf + 1) * 10;
        if arr[elf] >= target {
            return elf + 1;
        }
        for house in (elf..target / 10).step_by(elf + 1).skip(1) {
            arr[house] += (elf + 1) * 10;
        }
    }
    panic!("should have an answer")
}

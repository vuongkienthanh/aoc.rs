pub fn process(_input: &str) -> usize {
    let input: Vec<_> = _input
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let (card_pk, door_pk) = (input[0], input[1]);

    let mut loopsize = 0;
    let mut value = 1;
    loop {
        loopsize += 1;
        value = value * 7 % 20201227;
        if value == card_pk {
            println!("found card_loopsize first= {loopsize}");
            break (0..loopsize).fold(1, |acc, _| acc * door_pk % 20201227);
        }
        if value == door_pk {
            println!("found door_loopsize first= {loopsize}");
            break (0..loopsize).fold(1, |acc, _| acc * card_pk % 20201227);
        }
    }
}

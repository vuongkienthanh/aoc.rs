pub mod part1;
pub mod part2;
#[derive(Clone, Debug, Eq, PartialEq)]
enum Block {
    File { id: usize, count: usize },
    Free { count: usize },
}

fn checksum(input: &[Block]) -> usize {
    let mut sum = 0;
    let mut i = 0;

    for block in input {
        match block {
            Block::File { id, count } => {
                for _ in 0..*count {
                    sum += i * id;
                    i += 1;
                }
            }
            Block::Free { count } => i += count,
        }
    }

    sum
}

fn parse(input: &str) -> Vec<Block> {
    input
        .trim_end()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .enumerate()
        .map(|(i, x)| {
            if i % 2 == 0 {
                Block::File {
                    id: i / 2,
                    count: x,
                }
            } else {
                Block::Free { count: x }
            }
        })
        .collect::<Vec<_>>()
}

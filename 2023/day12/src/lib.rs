pub mod part1;
pub mod part2;

// A line in input
#[derive(Clone, PartialEq, Eq, Debug)]
struct HotSprings {
    springs: Vec<char>,
    records: Vec<usize>,
}

impl HotSprings {
    fn new(input: &str, expand: usize) -> Self {
        let (left, right) = input.split_once(' ').unwrap();
        // shorten continuous dots
        let mut springs = vec![];
        let mut leftpeek = left.chars().peekable();
        for _ in 0..left.len() {
            let cur = leftpeek.next().unwrap();
            if let Some(nxt) = leftpeek.peek() {
                if cur == '.' && nxt == &'.' {
                    continue;
                }
            }
            springs.push(cur);
        }
        let mut records = right
            .split(',')
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        if expand != 1 {
            let springs_len = springs.len();
            springs = springs
                .into_iter()
                .chain(std::iter::once('?'))
                .cycle()
                .take(springs_len * 5 + 4)
                .collect();
            let records_len = records.len();
            records = records
                .into_iter()
                .cycle()
                .take(records_len * 5)
                .collect();
        }


        HotSprings { springs, records }
    }


}



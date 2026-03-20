pub mod part1;
pub mod part2;
pub mod parsing;

#[derive(Debug, Clone)]
struct Node {
    next: usize,
    prev: usize,
}

fn build_hm(len: usize) -> Vec<Node> {
    let mut hm = vec![Node {
        next: 1,
        prev: len - 1,
    }];
    for i in 1..len - 1 {
        hm.push(Node {
            prev: i - 1,
            next: i + 1,
        });
    }
    hm.push(Node {
        prev: len - 2,
        next: 0,
    });
    hm
}

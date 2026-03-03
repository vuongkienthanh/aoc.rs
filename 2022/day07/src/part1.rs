use crate::parsing::parse_input;
use crate::{CD, CMD, LS};

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    println!("{input:?}");

    todo!("part1");
    // panic!("should have an answer")
}
fn get_size(node: &Rc<RefCell<Node>>, ans: &mut usize) -> usize {
    match node.borrow().data {
        LS::Dir(_) => {
            let size = node
                .borrow()
                .children
                .iter()
                .map(|x| get_size(x, ans))
                .sum::<usize>();
            if size <= 100_000 {
                *ans += size;
            }
            *ans
        }
        LS::File(u) => u,
    }
}
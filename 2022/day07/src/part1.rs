use crate::parsing::parse_input;
use crate::{CD, CMD, LS, build_tree, Node};
use std::cell::RefCell;
use std::rc::Rc;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let root = build_tree(input);
    let mut ans = 0;
    get_size(&root, &mut ans);
    ans
}

fn get_size(node: &Rc<RefCell<Node>>, ans: &mut usize) -> usize {
    let size = node.borrow().size
        + node
            .borrow()
            .children
            .iter()
            .map(|x| get_size(x, ans))
            .sum::<usize>();
    if size <= 100000 {
        *ans += size;
    }
    size
}
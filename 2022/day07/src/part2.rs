use crate::parsing::parse_input;
use crate::{CD, CMD, LS, build_tree, Node};
use std::cell::RefCell;
use std::rc::Rc;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let root = build_tree(input);
    let mut list_of_size = vec![];
    let used = get_size(&root, &mut list_of_size);
    let unused = 70_000_000 - used;
    let min_need_delete = 30_000_000 - unused;
    list_of_size.sort_unstable();
    list_of_size
        .into_iter()
        .find(|x| *x >= min_need_delete)
        .unwrap()
}

fn get_size(node: &Rc<RefCell<Node>>, list_of_size: &mut Vec<usize>) -> usize {
    let size = node.borrow().size
        + node
            .borrow()
            .children
            .iter()
            .map(|x| get_size(x, list_of_size))
            .sum::<usize>();
    list_of_size.push(size);
    size
}
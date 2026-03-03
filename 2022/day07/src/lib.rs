#![allow(non_camel_case_types)]
pub mod parsing;
pub mod part1;
pub mod part2;

#[derive(Debug)]
pub enum CMD<'a> {
    ls(Vec<LS<'a>>),
    cd(CD<'a>),
}

#[derive(Debug)]
pub enum CD<'a> {
    Back,
    Path(&'a str),
}

#[derive(Debug, Eq, PartialEq)]]
pub enum LS<'a> {
    Dir(&'a str),
    File(usize),
}

use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node<'a> {
    data: LS<'a>,
    parent: Option<Weak<RefCell<Node<'a>>>>,
    children: Vec<Rc<RefCell<Node<'a>>>>,
}

fn build_tree(input: Vec<CMD>) -> Rc<RefCell<Node>> {
    let root = Rc::new(RefCell::new(Node {
        data: LS::Dir("/"),
        parent: None,
        children: vec![],
    }));
    let mut current_node = Rc::clone(&root);
    for cmd in input {
        match cmd {
            CMD::cd(CD::Back) => {
                let mut parent = None;
                if let Some(p) = &current_node.borrow().parent {
                    parent = Some(p.clone());
                }
                if let Some(p) = parent {
                    current_node = p.upgrade().unwrap();
                }
            }
            CMD::cd(CD::Path(p)) => {
                let target = current_node
                    .borrow()
                    .children
                    .iter()
                    .find(|n| n.borrow().data == LS::Dir(p))
                    .map(|c| Rc::clone(c))
                    .unwrap();
                current_node = target;
            }
            CMD::ls(v) => {
                for child in v {
                    let child = Node {
                        data: child,
                        parent: Some(Rc::downgrade(&current_node)),
                        children: vec![],
                    };
                    current_node
                        .borrow_mut()
                        .children
                        .push(Rc::new(RefCell::new(child)));
                }
            }
        }
    }
    root
}


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

#[derive(Debug)]]
pub enum LS<'a> {
    Dir(&'a str),
    File(usize),
}

use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node<'a> {
    name: &'a str,
    size: usize,
    parent: Option<Weak<RefCell<Node<'a>>>>,
    children: Vec<Rc<RefCell<Node<'a>>>>,
}

fn build_tree(input: Vec<CMD>) -> Rc<RefCell<Node>> {
    let root = Rc::new(RefCell::new(Node {
        name: "/",
        size: 0,
        parent: None,
        children: vec![],
    }));
    let mut current_node = Rc::clone(&root);
    for cmd in input {
        match cmd {
            CMD::cd(CD::Back) => {
                let p = current_node.borrow().parent.as_ref().unwrap().upgrade().unwrap();
                current_node = p;
            }
            CMD::cd(CD::Path(p)) => {
                let target = current_node
                    .borrow()
                    .children
                    .iter()
                    .find(|n| n.borrow().name == p)
                    .map(|c| Rc::clone(c))
                    .unwrap();
                current_node = target;
            }
            CMD::ls(v) => {
                for child in v {
                    match child {
                        LS::Dir(x) => {
                            let child = Node {
                                name: x,
                                size: 0,
                                parent: Some(Rc::downgrade(&current_node)),
                                children: vec![],
                            };
                            current_node
                                .borrow_mut()
                                .children
                                .push(Rc::new(RefCell::new(child)));
                        }
                        LS::File(x) => {
                            current_node.borrow_mut().size += x;
                        }
                    }
                }
            }
        }
    }
    root
}
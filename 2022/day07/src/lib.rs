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

#[derive(Debug)]
pub enum LS<'a> {
    Dir(&'a str),
    File(usize),
}

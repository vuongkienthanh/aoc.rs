pub mod parsing;
pub mod part1;
pub mod part2;

use aoc_helper::assembly::{Command, Computer};
use parsing::{Item, Target};

impl Command for Item {
    fn run<const N: usize, T>(self, computer: &mut Computer<N, T>)
    where
        T: Command,
    {
        match self {
            Item::cpy(target, register) => {
                match target {
                    Target::Value(v) => computer.registers[register] = v,
                    Target::Register(r) => computer.registers[register] = computer.registers[r],
                };
                computer.i += 1;
            }
            Item::inc(register) => {
                computer.registers[register] += 1;
                computer.i += 1;
            }
            Item::dec(register) => {
                computer.registers[register] -= 1;
                computer.i += 1;
            }
            Item::jnzf(target, x) => match target {
                Target::Value(v) => {
                    if v != 0 {
                        computer.i += x
                    } else {
                        computer.i += 1;
                    }
                }
                Target::Register(r) => {
                    if computer.registers[r] != 0 {
                        computer.i += x
                    } else {
                        computer.i += 1;
                    }
                }
            },
            Item::jnzb(target, x) => match target {
                Target::Value(v) => {
                    if v != 0 {
                        computer.i -= x
                    } else {
                        computer.i += 1;
                    }
                }
                Target::Register(r) => {
                    if computer.registers[r] != 0 {
                        computer.i -= x
                    } else {
                        computer.i += 1;
                    }
                }
            },
        }
    }
}

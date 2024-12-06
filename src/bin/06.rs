#![allow(unused)]

use aoc_2024 as aoc;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let in1 = include_str!("../../data/06/in1");
    println!("pt1: {}", aoc::timed(|| eval(in1, true)));
    println!("pt2: {}", aoc::timed(|| eval(in1, false)));
}

fn eval(s: &str, pt1: bool) -> usize {
    todo!("eval")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex01() {
        let s = include_str!("../../data/06/ex1");
        assert_eq!(eval(s, true), 143);
    }
}

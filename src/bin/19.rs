#![allow(unused)]

use hashbrown::{
    HashMap, HashSet,
    hash_map::Entry::{Occupied, Vacant},
};

#[allow(unused)]
mod data {
    pub static EX1: &str = include_str!("../../data/19/ex1");
    pub static IN1: &str = include_str!("../../data/19/in1");
}

fn main() {
    println!("pt1: {}", aoc_2024::timed(|| eval(data::IN1, true)));
    //println!("pt2: {}", aoc_2024::timed(|| eval(data::IN1, false)));
}

fn eval(s: &str, pt1: bool) -> usize {
    let prob = parse(s);
    prob.valid_designs()
}

#[derive(Clone, Hash, PartialEq, Eq, Default)]
struct Colors<'a>(&'a [u8]);

#[derive(Debug, Clone)]
struct Problem<'a> {
    patterns: Vec<Colors<'a>>,
    designs: Vec<Colors<'a>>,
}

impl<'a> Problem<'a> {
    fn possible_designs(&self) -> usize {
        self.designs
            .iter()
            .map(|c| self.valid_design(c))
            .sum()
    }
    fn valid_designs(&self) -> usize {
        self.designs
            .iter()
            .filter(|c| self.valid_design(c) > 0)
            .count()
    }
    fn valid_design(&self, design: &Colors) -> usize {
        type Mem<'a> = HashMap<Colors<'a>, usize>;
        type Stack<'a> = Vec<Colors<'a>>;
        fn search<'a, 'b>(
            this: &'a Problem,
            design: &'b Colors<'a>,
            stack: &mut Stack<'a>,
            solutions: &mut HashSet<Stack<'a>>,
            mem: &mut Mem<'a>,
        ) -> usize {
            if design.is_empty() {
                return 1;
            }
            let mut sum = 0;
            for pat in &this.patterns {
                let Some(rest) = design.strip_prefix(pat) else { continue };
                if let Some(val) = mem.get(&rest) {
                    sum + val;
                    continue;
                }
                let val = search(this, &rest, stack, solutions, mem);
                println!("{rest:?} -> {val}");
                mem.insert(rest, val);
                sum += val;
            }
            sum
        }
        let mut mem = Mem::new();
        let mut stack = Stack::new();
        let mut solutions = HashSet::new();
        search(&self, design, &mut stack, &mut solutions, &mut mem)
    }
}

impl<'a> Colors<'a> {
    fn strip_prefix(&self, other: &Self) -> Option<Self> {
        self.0.strip_prefix(other.0).map(Colors)
    }
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

fn parse(s: &str) -> Problem {
    let (p1, p2) = s.trim().split_once("\n\n").unwrap();
    let patterns = p1
        .split(", ")
        .map(|s| Colors(s.as_bytes()))
        .collect();
    let designs = p2.lines().map(|s| Colors(s.as_bytes())).collect();
    Problem { patterns, designs }
}

impl<'a> std::fmt::Debug for Colors<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.0).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(eval(data::EX1, true), 6);
    }

    #[test]
    fn pt1() {
        assert_eq!(eval(data::IN1, true), 265);
    }

    #[test]
    fn ex2() {
        assert_eq!(eval(data::EX1, false), 16);
    }
}

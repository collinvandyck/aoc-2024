#![allow(unused)]

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

#[derive(Clone)]
struct Colors<'a>(&'a [u8]);

#[derive(Debug, Clone)]
struct Problem<'a> {
    patterns: Vec<Colors<'a>>,
    designs: Vec<Colors<'a>>,
}

impl<'a> Problem<'a> {
    fn valid_designs(&self) -> usize {
        self.designs
            .iter()
            .filter(|c| self.valid_design(c))
            .count()
    }
    fn valid_design(&self, design: &Colors) -> bool {
        if design.is_empty() {
            return true;
        }
        self.patterns.iter().any(|p| {
            design
                .strip_prefix(p)
                .map(|rest| self.valid_design(&rest))
                .unwrap_or_default()
        })
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
}

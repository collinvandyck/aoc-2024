#![allow(unused)]

#[allow(unused)]
mod data {
    pub static EX1: &str = include_str!("../../data/19/ex1");
    pub static IN1: &str = include_str!("../../data/19/in1");
}

fn main() {
    println!("pt1: {}", aoc_2024::timed(|| eval(data::IN1, true)));
    println!("pt2: {}", aoc_2024::timed(|| eval(data::IN1, false)));
}

fn eval(s: &str, pt1: bool) -> usize {
    let prob = parse(s);
    prob.valid_designs()
}

#[derive(Clone)]
struct Colors(Vec<u8>);

#[derive(Debug, Clone)]
struct Problem {
    patterns: Vec<Colors>,
    designs: Vec<Colors>,
}

impl Problem {
    fn valid_designs(&self) -> usize {
        0
    }
}

fn parse(s: &str) -> Problem {
    let (p1, p2) = s.trim().split_once("\n\n").unwrap();
    let patterns = p1
        .split(", ")
        .map(|s| s.bytes().collect())
        .collect();
    let designs = p2.lines().map(|s| s.bytes().collect()).collect();
    Problem { patterns, designs }
}

impl FromIterator<u8> for Colors {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl std::fmt::Debug for Colors {
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

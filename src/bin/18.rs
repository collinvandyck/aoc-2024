#![allow(unused)]

use itertools::Itertools;

#[allow(unused)]
mod data {
    pub static EX1: &str = include_str!("../../data/18/ex1");
    pub static IN1: &str = include_str!("../../data/18/in1");
}

fn main() {
    println!("pt1: {}", aoc_2024::timed(|| eval(data::IN1, false, true)));
    // println!("pt2: {}", aoc_2024::timed(|| eval(data::IN1, false)));
}

fn eval(s: &str, ex: bool, pt1: bool) -> usize {
    let bytes = parse(s);
    let dim = ex.then_some(7).unwrap_or(71);
    let mut grid = Grid { dim, bytes };
    grid.solve()
}

struct Grid {
    dim: i32,
    bytes: Vec<(i32, i32)>,
}

impl Grid {
    fn solve(&mut self) -> usize {
        todo!("solve")
    }
}

fn parse(s: &str) -> Vec<(i32, i32)> {
    s.trim()
        .lines()
        .flat_map(|l| l.split_once(","))
        .map(|f| (f.0.parse().unwrap(), f.1.parse().unwrap()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(eval(data::EX1, true, true), 22);
    }
}

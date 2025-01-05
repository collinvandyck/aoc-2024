use core::panic;
use std::collections::{HashMap, HashSet, VecDeque};

#[allow(unused)]
mod data {
    pub static EX1: &str = include_str!("../../data/18/ex1");
    pub static IN1: &str = include_str!("../../data/18/in1");
}

fn main() {
    println!("pt1: {}", aoc_2024::timed(|| shortest(data::IN1, false)));
    println!("pt2: {}", aoc_2024::timed(|| breaking_byte(data::IN1, false)));
}

fn shortest(s: &str, ex: bool) -> usize {
    let bytes = parse(s);
    let dim = if ex { 7 } else { 71 };
    let bytes = bytes
        .into_iter()
        .take(ex.then_some(12).unwrap_or(1024))
        .collect();
    let mut grid = Grid::new(dim, bytes);
    grid.shortest().unwrap()
}

fn breaking_byte(s: &str, ex: bool) -> String {
    let bytes = parse(s);
    let dim = ex.then_some(7).unwrap_or(71);
    let mut grid = Grid::new(dim, bytes);
    grid.first_breaking_byte()
}

#[derive(Clone)]
struct Grid {
    dim: i32,
    bytes: VecDeque<Pos>,
}

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
struct Pos(i32, i32);

impl Grid {
    fn first_breaking_byte(&mut self) -> String {
        for i in 0..self.bytes.len() {
            let mut g = Grid {
                dim: self.dim,
                bytes: self.bytes.iter().take(i + 1).copied().collect(),
            };
            let shortest = g.shortest();
            if shortest.is_none() {
                let b = self.bytes.get(i).unwrap();
                println!("{i:2} {:?}", self.bytes[i]);
                return format!("{},{}", b.1, b.0);
            }
        }
        panic!("nothing broke it.");
    }
    fn shortest(&mut self) -> Option<usize> {
        let mut corrupted = HashSet::<Pos>::with_capacity(self.dim as usize * 2);
        let (start, end) = (Pos(0, 0), Pos(self.dim - 1, self.dim - 1));
        let mut costs = HashMap::<Pos, usize>::with_capacity(self.dim as usize * 2);
        let mut stack = Vec::with_capacity(self.dim as usize * 10);
        stack.push(Vec::with_capacity(self.dim as usize * 10));
        stack.last_mut().unwrap().push(start);
        while let Some(b) = self.bytes.pop_front() {
            corrupted.insert(b);
        }
        loop {
            let mut nexts = Vec::with_capacity(stack.last().unwrap().len() * 2);
            for &cur in stack.last().unwrap() {
                costs.insert(cur, stack.len());
                if cur == end {
                    continue;
                }
                let ds = [(1, 0), (-1, 0), (0, 1), (0, -1)];
                for ds in ds {
                    let next = Pos(cur.0 + ds.0, cur.1 + ds.1);
                    if nexts.contains(&next) {
                        continue;
                    }
                    if costs.contains_key(&next) {
                        continue;
                    }
                    if next.0 < 0 || next.1 < 0 || next.0 >= self.dim || next.1 >= self.dim {
                        continue;
                    }
                    if corrupted.contains(&next) {
                        continue;
                    }
                    nexts.push(next);
                }
            }
            if nexts.is_empty() {
                break;
            }
            stack.push(nexts);
        }
        costs.get(&end).map(|c| *c - 1)
    }
    fn new(dim: i32, bytes: Vec<Pos>) -> Self {
        Self {
            dim,
            bytes: VecDeque::from(bytes),
        }
    }
}

fn parse(s: &str) -> Vec<Pos> {
    s.trim()
        .lines()
        .flat_map(|l| l.split_once(","))
        .map(|t| (t.0.parse().unwrap(), t.1.parse().unwrap()))
        .map(|t| Pos(t.1, t.0)) // XY -> R/C
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(shortest(data::EX1, true), 22);
    }

    #[test]
    fn pt1() {
        assert_eq!(shortest(data::IN1, false), 334);
    }

    #[test]
    fn ex2() {
        assert_eq!(breaking_byte(data::EX1, true), "6,1");
    }

    #[test]
    fn pt2() {
        assert_eq!(breaking_byte(data::IN1, false), "20,12");
    }
}

#![allow(unused)]

#[cfg(test)]
static EX1: &str = include_str!("../../data/12/ex1");
static IN1: &str = include_str!("../../data/12/in1");

fn main() {
    println!("pt1: {}", aoc_2024::timed(|| eval(IN1, true)));
    println!("pt2: {}", aoc_2024::timed(|| eval(IN1, false)));
}

fn eval(s: &str, pt1: bool) -> usize {
    0
}

#[derive(Default)]
struct Grid {
    tiles: Vec<Vec<Tile>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Tile {
    row: i32,
    col: i32,
    val: u32,
}

impl Grid {
    fn get(&self, r: i32, c: i32) -> Option<Tile> {
        (r >= 0 && c >= 0)
            .then_some((r as usize, c as usize))
            .and_then(|(r, c)| self.tiles.get(r)?.get(c).copied())
    }
    fn parse(s: &str) -> Self {
        let tiles = s
            .trim()
            .lines()
            .enumerate()
            .map(|(row, l)| {
                l.chars()
                    .enumerate()
                    .map(|(col, ch)| {
                        let val = ch.to_digit(10).unwrap();
                        let (row, col) = (row as i32, col as i32);
                        Tile { row, col, val }
                    })
                    .collect()
            })
            .collect();
        Self { tiles }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exs() {
        assert_eq!(eval(EX1, true), 55312);
    }
}

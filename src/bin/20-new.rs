#![allow(unused)]

use std::{collections::HashSet, ops::Add};

#[allow(unused)]
mod data {
    pub static EX1: &str = include_str!("../../data/20/ex1");
    pub static IN1: &str = include_str!("../../data/20/in1");
}

fn main() {
    println!("pt1: {}", aoc_2024::timed(|| eval(data::IN1, true, false)));
}

fn eval(s: &str, pt1: bool, ex: bool) -> usize {
    let grid = Grid::parse(s);
    todo!("eval")
}

struct Grid {
    dim: i32,
    tiles: Vec<Tile>,
}

impl Grid {
    fn path(&self) -> Vec<Tile> {
        let cur = self.tiles.iter().find(|t| t.ch == b'S');
        let cur = cur.copied().unwrap();
        let visited = HashSet::from([cur]);
        let dirs = [Pt(0, 1), Pt(1, 0), Pt(0, -1), Pt(-1, 0)];
        while cur.ch != b'E' {
            let nexts = dirs.iter().flat_map(|&pt| self.next(cur, pt));
            let nexts = nexts.filter(|t| t.ch != b'#');
            for next in nexts {
                println!("next: {next:?}");
            }
            break;
        }
        println!("{cur:?}");
        todo!()
    }
    fn next(&self, tile: Tile, dir: Pt) -> Option<Tile> {
        let pt = tile.pt + dir;
        self.get(pt)
    }
    fn get(&self, pt: Pt) -> Option<Tile> {
        let Pt(row, col) = pt;
        (row >= 0 && col >= 0 && row < self.dim && col < self.dim)
            .then(|| {
                let (row, col) = (row as usize, col as usize);
                self.tiles.get(row * self.dim as usize + col)
            })
            .flatten()
            .copied()
    }
    fn parse(s: &str) -> Grid {
        let tiles: Vec<Vec<Tile>> = s
            .trim()
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.trim()
                    .as_bytes()
                    .iter()
                    .enumerate()
                    .map(|(col, &ch)| {
                        Tile {
                            pt: Pt(row as i32, col as i32),
                            ch,
                        }
                    })
                    .collect()
            })
            .collect();
        assert_eq!(tiles.len(), tiles[0].len());
        let dim = tiles.len() as i32;
        let tiles = tiles.into_iter().flatten().collect();
        Grid { dim, tiles }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Tile {
    pt: Pt,
    ch: u8,
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch: char = self.ch.into();
        write!(f, "{} ({},{})", ch, self.pt.0, self.pt.1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pt(i32, i32);

impl std::ops::Add for Pt {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_ex1() {
        let grid = Grid::parse(data::EX1);
        let path = grid.path();
        println!("{path:?}");
        assert!(false);
    }
}

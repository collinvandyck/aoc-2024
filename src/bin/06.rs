#![allow(unused)]

use aoc_2024 as aoc;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let in1 = include_str!("../../data/06/in1");
    println!("pt1: {}", aoc::timed(|| eval(in1, true)));
}

fn eval(s: &str, pt1: bool) -> usize {
    let mut grid = Grid::from_str(s);
    grid.unique(pt1)
}

struct Grid {
    tiles: Vec<Vec<Tile>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Tile(usize, usize, char);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct TileDir(Tile, usize);

const DIRS: [char; 4] = ['U', 'R', 'D', 'L'];

impl Grid {
    fn unique(&mut self, pt1: bool) -> usize {
        let mut cur = self.find('^').unwrap();
        let mut dir = DIRS.iter().position(|&c| c == 'U').unwrap();
        let mut vis = HashMap::from([(cur, TileDir(cur, dir))]);
        loop {
            let next = self.tile_for_dir(cur, dir);
            match next {
                Some(Tile(_, _, '#')) => dir = (dir + 1) % DIRS.len(),
                Some(tile) => {
                    cur = tile;
                    vis.insert(tile, TileDir(tile, dir));
                }
                None => break,
            }
        }
        pt1.then_some(vis.len()).unwrap_or(0)
    }
    fn find(&self, ch: char) -> Option<Tile> {
        self.flatten().find(|t| t.2 == ch)
    }
    fn flatten(&self) -> impl Iterator<Item = Tile> + '_ {
        self.tiles.iter().flatten().copied()
    }
    fn tile_for_dir(&self, tile: Tile, dir: usize) -> Option<Tile> {
        let (rows, cols) = self.rows_cols();
        let (r, c) = match DIRS[dir] {
            'U' => (tile.0.checked_sub(1), Some(tile.1)),
            'D' => ((tile.0 < rows - 1).then_some(tile.0 + 1), Some(tile.1)),
            'L' => (Some(tile.0), tile.1.checked_sub(1)),
            'R' => (Some(tile.0), ((tile.1 < cols - 1).then_some(tile.1 + 1))),
            _ => unreachable!(),
        };
        r.and_then(|r| c.and_then(|c| self.get(r, c)))
    }
    fn get(&self, r: usize, c: usize) -> Option<Tile> {
        self.tiles.get(r).and_then(|r| r.get(c)).copied()
    }
    fn rows_cols(&self) -> (usize, usize) {
        (self.tiles.len(), self.tiles[0].len())
    }
    fn from_str(s: &str) -> Self {
        let tiles = s
            .trim()
            .lines()
            .enumerate()
            .map(|(row, l)| {
                l.chars()
                    .enumerate()
                    .map(|(col, ch)| Tile(row, col, ch))
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
    fn pt1() {
        let s = include_str!("../../data/06/in1");
        assert_eq!(eval(s, true), 5564);
    }

    #[test]
    fn ex01() {
        let s = include_str!("../../data/06/ex1");
        assert_eq!(eval(s, true), 41);
    }

    #[test]
    fn ex02() {
        let s = include_str!("../../data/06/ex1");
        assert_eq!(eval(s, false), 6);
    }
}

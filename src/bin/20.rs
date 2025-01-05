#![allow(unused)]

use bitvec::{order::Msb0, slice::BitSlice, vec::BitVec};
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::RandomState,
    ops::Add,
    time::Instant,
    vec,
};

#[allow(unused)]
mod data {
    pub static EX1: &str = include_str!("../../data/20/ex1");
    pub static IN1: &str = include_str!("../../data/20/in1");
}

fn main() {
    println!(
        "pt1: {}",
        aoc_2024::timed(|| num_cheats_saving_at_least(data::IN1, 100))
    );
}

fn num_cheats_saving_at_least(s: &str, threshold: usize) -> usize {
    let mut grid = Grid::parse(s);
    let state = shortest_path(&grid);
    let mut savings = HashMap::<Pt, usize>::new();
    for (idx, &path_tile) in state.tiles.iter().enumerate() {
        let mut nexts = NEXTS
            .into_iter()
            .filter_map(|pt| grid.next(path_tile, pt))
            .filter(|t| t.is_wall())
            .flat_map(|first| {
                NEXTS
                    .into_iter()
                    .filter_map(|pt| {
                        grid.next(first, pt)
                            .filter(|t| !t.is_wall())
                            .map(|second| (first, second))
                    })
                    .collect_vec()
                    .into_iter()
            })
            .fold(HashSet::<Tile>::new(), |mut acc, (first, _)| {
                acc.insert(first);
                acc
            });
        for next in nexts {
            if savings.contains_key(&next.pt) {
                continue;
            }
            let prev = grid.set(next.pt, b'.');
            let test = shortest_path(&grid);
            let diff = test.tiles.len().abs_diff(state.tiles.len());
            savings.insert(next.pt, diff);
            grid.set(prev.pt, prev.ch);
        }
    }
    savings
        .values()
        .fold(HashMap::<usize, usize>::new(), |mut acc, v| {
            *acc.entry(*v).or_default() += 1;
            acc
        })
        .into_iter()
        .filter_map(|(savings, count)| (savings >= threshold).then_some(count))
        .sum()
}

#[derive(Clone)]
struct Grid {
    dim: i16,
    tiles: Vec<Tile>,
    start: Tile,
}

static NEXTS: [Pt; 4] = [Pt(0, 1), Pt(1, 0), Pt(0, -1), Pt(-1, 0)];

#[derive(Clone, Default)]
struct State {
    dim: usize,
    tiles: Vec<Tile>,
    visited: Vec<bool>,
}
impl State {
    fn new(dim: usize) -> Self {
        Self {
            dim,
            tiles: Vec::with_capacity(dim * 2),
            visited: vec![false; (dim * dim)],
        }
    }
    fn push(&mut self, tile: Tile) {
        self.tiles.push(tile);
        let idx = tile.pt.idx(self.dim);
        self.visited[idx] = true;
    }
    fn has_visited(&self, pt: Pt) -> bool {
        let idx = pt.idx(self.dim);
        self.visited[idx]
    }
}

fn shortest_path(grid: &Grid) -> State {
    let epoch = Instant::now();
    let f = bitvec::bits![u16, bitvec::order::Msb0; 0; 32];
    let cur = grid.start;
    let mut state = State::new(grid.dim as usize);
    state.push(cur);
    let mut queue = VecDeque::from([state]);
    let mut best = State::default();
    let mut best_cost = usize::MAX;
    while !queue.is_empty() {
        let mut state = queue.pop_front().unwrap();
        let cur = state.tiles.last().copied().unwrap();
        if best_cost < state.tiles.len() {
            continue;
        }
        if cur.is_end() {
            best_cost = state.tiles.len();
            best = state;
            continue;
        }
        let mut nexts = NEXTS
            .into_iter()
            .filter_map(|pt| grid.next(cur, pt))
            .filter(|t| !t.is_wall() && !state.has_visited(t.pt));
        if let Some(next) = nexts.next() {
            for next in nexts {
                let mut state = state.clone();
                state.push(next);
                queue.push_front(state);
            }
            state.push(next);
            queue.push_front(state);
        }
    }
    //println!("state: {} {:?}", best.tiles.len(), epoch.elapsed());
    best
}

impl Grid {
    fn find(&self, ch: u8) -> Tile {
        self.tiles
            .iter()
            .find(|t| t.ch == ch)
            .copied()
            .unwrap()
    }
    #[inline(always)]
    fn next(&self, tile: Tile, dir: Pt) -> Option<Tile> {
        let pt = tile.pt + dir;
        self.get(pt)
    }
    #[inline(always)]
    fn get(&self, pt: Pt) -> Option<Tile> {
        let Pt(row, col) = pt;
        if row >= 0 && col >= 0 && row < self.dim && col < self.dim {
            let (row, col) = (row as usize, col as usize);
            let idx = row * self.dim as usize + col;
            self.tiles.get(idx).copied()
        } else {
            None
        }
    }
    #[inline(always)]
    fn idx(&self, pt: Pt) -> usize {
        pt.idx(self.dim as usize)
    }
    #[inline(always)]
    fn set(&mut self, pt: Pt, ch: u8) -> Tile {
        let idx = self.idx(pt);
        let prev = self.tiles[idx];
        self.tiles[idx] = Tile { pt, ch };
        prev
    }
    fn parse(s: &str) -> Grid {
        let mut start: Option<Tile> = None;
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
                        let t = Tile {
                            pt: Pt(row as i16, col as i16),
                            ch,
                        };
                        if ch == b'S' {
                            start.replace(t);
                        }
                        t
                    })
                    .collect()
            })
            .collect();
        assert_eq!(tiles.len(), tiles[0].len());
        let dim = tiles.len() as i16;
        let tiles = tiles.into_iter().flatten().collect();
        Grid {
            dim,
            tiles,
            start: start.unwrap(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Tile {
    pt: Pt,
    ch: u8,
}

impl Tile {
    #[inline(always)]
    fn is_wall(&self) -> bool {
        self.ch == b'#'
    }
    #[inline(always)]
    fn is_end(&self) -> bool {
        self.ch == b'E'
    }
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch: char = self.ch.into();
        write!(f, "{} ({},{})", ch, self.pt.0, self.pt.1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pt(i16, i16);

impl Pt {
    #[inline(always)]
    fn idx(&self, dim: usize) -> usize {
        self.0 as usize * dim + self.1 as usize
    }
}

impl std::hash::Hash for Pt {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let v = ((self.0 as i32) << 16) | (self.1 as i32 & 0xFFFF);
        state.write_i32(v);
    }
}

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
        let cheats = num_cheats_saving_at_least(data::EX1, 20);
        assert_eq!(cheats, 5);
    }
}

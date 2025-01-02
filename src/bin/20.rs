#![allow(unused)]

use fnv::FnvHashMap;
use hashbrown::{DefaultHashBuilder, HashMap, HashSet};
use itertools::{Itertools, iproduct};
use priority_queue::PriorityQueue;
use std::{self, cmp::Reverse, collections::VecDeque, hash::RandomState, ops::Add, time::Instant, usize};

#[allow(unused)]
mod data {
    pub static EX1: &str = include_str!("../../data/20/ex1");
    pub static IN1: &str = include_str!("../../data/20/in1");
}

fn main() {
    println!("pt1: {}", aoc_2024::timed(|| eval(data::IN1, true, false)));
    //println!("pt2: {}", aoc_2024::timed(|| eval(data::IN1, false)));
}

fn eval(s: &str, pt1: bool, ex: bool) -> usize {
    let grid = parse(s);
    let cheats = grid.cheats();
    cheats
        .iter()
        .filter(|c| c.savings >= if ex { 12 } else { 100 })
        .count()
}

static DIRS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

#[derive(Clone)]
struct Grid {
    tiles: Vec<Vec<Tile>>,
}

#[derive(Default)]
struct Costs(Option<(Vec<Tile>, HashMap<Tile, Vec<Tile>>)>);

impl Costs {
    fn best(&self) -> Option<usize> {
        self.0.as_ref().map(|(ts, _)| ts.len())
    }
}

impl Grid {
    fn cheats(&self) -> Vec<Cheat> {
        let start = Instant::now();
        let path: Vec<Tile> = self.bfs(&Costs::default());
        let costs = Costs(Some((
            path.clone(),
            (0..path.len() - 1)
                .map(|i| (path[i], (&path[i..]).to_vec()))
                .collect(),
        )));
        println!("bfs: {:?} {}", start.elapsed(), path.len());
        Vec::default()
    }
    fn bfs(&self, costs: &Costs) -> Vec<Tile> {
        let epoch = Instant::now();
        struct Path(Vec<Tile>, HashSet<Tile>);
        let start = self.start();
        let start = Path(vec![start], HashSet::from([start]));
        let mut paths = VecDeque::from([start]);
        let mut best = Option::<Vec<Tile>>::None;
        while !paths.is_empty() {
            for pidx in 0..paths.len() {
                let Path(mut tiles, mut cache) = paths.pop_front().unwrap();
                let cost = tiles.len();
                let cur = tiles.last().unwrap();
                if cur.is_end() {
                    best = match best {
                        Some(b) if b.len() < tiles.len() => Some(b),
                        _ => Some(tiles),
                    };
                    continue;
                }
                // discard if our cost is already over a recorded local best
                if matches!(&best, Some(b) if b.len() <= cost) {
                    continue;
                }
                // discard if our cost is already over a previous recorded best
                if costs.best().map(|b| b <= cost).unwrap_or(false) {
                    continue;
                }
                // what if we encounter a node our Costs has already encountered

                // find the next paths
                let mut nexts = DIRS
                    .into_iter()
                    .flat_map(|d| self.next(*cur, d))
                    .filter(|t| !t.is_wall() && !cache.contains(t))
                    .collect_vec();
                if let [xs @ .., x] = nexts.as_slice() {
                    if xs.len() > 0 {
                        println!("cloning for {}", xs.len());
                        for x in xs {
                            let mut tiles = tiles.clone();
                            let mut cache = cache.clone();
                            tiles.push(*x);
                            cache.insert(*x);
                            paths.push_back(Path(tiles, cache));
                        }
                    }
                    tiles.push(*x);
                    cache.insert(*x);
                    paths.push_back(Path(tiles, cache));
                }
            }
        }
        best.or_else(|| costs.0.as_ref().map(|(ts, _)| ts.clone()))
            .unwrap()
    }
    fn dijkstra(&self) -> Vec<Tile> {
        type Queue = PriorityQueue<Tile, Reverse<usize>, hashbrown::DefaultHashBuilder>;
        let epoch = Instant::now();
        let dim = self.tiles.len() * self.tiles[0].len();
        let mut queue = Queue::with_capacity_and_default_hasher(dim);
        let mut prev = FnvHashMap::<Tile, Tile>::with_capacity_and_hasher(dim, Default::default());
        let mut dist = FnvHashMap::<Tile, usize>::with_capacity_and_hasher(dim, Default::default());
        let start = self.start();
        self.tiles()
            .filter(|t| !t.is_wall())
            .for_each(|tile| {
                if tile == start {
                    dist.insert(tile, 0);
                    queue.push(tile, Reverse(0));
                } else {
                    queue.push(tile, Reverse(usize::MAX));
                    dist.insert(tile, usize::MAX);
                }
            });
        let epoch = Instant::now();
        let mut best = Option::<usize>::None;
        while let Some((u, Reverse(cost))) = queue.pop() {
            if u.is_end() {
                if best.is_none() || best.unwrap_or_default() > cost {
                    best.replace(cost);
                }
                continue;
            }
            if matches!(best, Some(c) if c <= cost) {
                continue;
            }
            for dir in DIRS {
                let &u_dist = dist.get(&u).unwrap();
                if let Some(v) = self.next(u, dir).filter(|t| !t.is_wall()) {
                    let &v_dist = dist.get(&v).unwrap();
                    let alt = u_dist + 1;
                    if alt < v_dist {
                        prev.insert(v, u);
                        dist.insert(v, alt);
                        queue.change_priority(&v, Reverse(alt));
                    }
                }
            }
        }
        let end = self.tiles().find(|t| t.is_end()).unwrap();
        let mut tiles = vec![end];
        tiles.push(end);
        loop {
            let last = tiles.last().unwrap();
            let Some(&prev) = prev.get(last) else {
                break;
            };
            tiles.push(prev);
        }
        tiles.reverse();
        tiles
    }

    fn next(&self, tile: Tile, dlt: (i32, i32)) -> Option<Tile> {
        self.get_pt(tile.pt + dlt)
    }
    fn start(&self) -> Tile {
        self.tiles().find(Tile::is_start).unwrap()
    }
    fn tiles(&self) -> impl Iterator<Item = Tile> + '_ {
        self.tiles.iter().flatten().copied()
    }
    fn get_pt(&self, pt: Pt) -> Option<Tile> {
        self.get(pt.0, pt.1)
    }
    fn get(&self, r: i32, c: i32) -> Option<Tile> {
        (r >= 0 && c >= 0)
            .then(|| {
                let row = self.tiles.get(r as usize);
                row.and_then(|row| row.get(c as usize).copied())
            })
            .flatten()
    }
    fn print(&self, path: &[Tile]) {
        for r in &self.tiles {
            for tile in r.iter() {
                if path.contains(tile) {
                    print!("x");
                } else {
                    print!("{}", tile.ch)
                }
            }
            println!();
        }
        println!();
    }
}

struct Cheat {
    path: Path,
    start: Pt,
    end: Pt,
    savings: usize,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
struct Path(Vec<Tile>);

impl Path {
    fn push(&mut self, tile: Tile) {
        self.0.push(tile);
    }
    fn tiles(&self) -> &[Tile] {
        &self.0
    }
    fn cost(&self) -> usize {
        self.0.len() - 1
    }
}

impl Path {
    fn new<'a>(stack: impl IntoIterator<Item = &'a Tile>) -> Self {
        Self(stack.into_iter().copied().collect())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pt(i32, i32);

impl std::ops::Add<(i32, i32)> for Pt {
    type Output = Pt;
    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Add for Pt {
    type Output = Pt;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Tile {
    pt: Pt,
    ch: char,
}

impl Tile {
    fn is_start(&self) -> bool {
        self.ch == 'S'
    }
    fn is_end(&self) -> bool {
        self.ch == 'E'
    }
    fn is_wall(&self) -> bool {
        self.ch == '#'
    }
}

fn parse(s: &str) -> Grid {
    let tiles = s
        .trim()
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(|(col, ch)| Tile {
                    pt: Pt(row as i32, col as i32),
                    ch,
                })
                .collect()
        })
        .collect();
    Grid { tiles }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_speeds() {
        // find the number of cheats that save at least 12 picoseconds
        assert_eq!(eval(data::EX1, true, true), 8);
        //   assert_eq!(eval(data::EX1, true, true), 42);
    }

    //#[test]
    //fn ex1() {
    // find the number of cheats that save at least 12 picoseconds
    //assert_eq!(eval(data::EX1, true, true), 8);
    //}
}

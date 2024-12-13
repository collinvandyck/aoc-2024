use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

#[allow(unused)]
mod data {
    pub static EX1: &str = include_str!("../../data/12/ex1");
    pub static EX2: &str = include_str!("../../data/12/ex2");
    pub static EX3: &str = include_str!("../../data/12/ex3");
    pub static EX4: &str = include_str!("../../data/12/ex4");
    pub static EX5: &str = include_str!("../../data/12/ex5");
    pub static IN1: &str = include_str!("../../data/12/in1");
}

fn main() {
    println!("pt1: {}", aoc_2024::timed(|| eval(data::IN1, true)));
    println!("pt2: {}", aoc_2024::timed(|| eval(data::IN1, false)));
}

fn eval(s: &str, pt1: bool) -> usize {
    Grid::parse(s).price(!pt1)
}

#[derive(Default)]
struct Grid {
    tiles: Vec<Vec<Tile>>,
    rows: i32,
    cols: i32,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
struct Tile {
    row: i32,
    col: i32,
    ch: char,
}

#[derive(Debug, Default)]
struct Region {
    plots: Vec<Plot>,
}

#[derive(Debug, PartialEq, Eq)]
struct Plot {
    tile: Tile,
    borders: Vec<Dir>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Region {
    fn add(&mut self, plot: Plot) {
        self.plots.push(plot);
    }
    fn contains(&self, tile: &Tile) -> bool {
        self.plots.iter().any(|p| &p.tile == tile)
    }
    fn price(&self, bulk: bool) -> usize {
        if !bulk {
            self.area() * self.perimeter()
        } else {
            self.area() * self.sides()
        }
    }
    fn area(&self) -> usize {
        self.plots.len()
    }
    fn perimeter(&self) -> usize {
        self.plots.iter().map(|p| p.borders.len()).sum()
    }
    fn sides(&self) -> usize {
        self.plots
            .iter()
            .flat_map(|plot| plot.borders.iter().map(|dir| (*dir, plot.tile)))
            .into_group_map()
            .into_iter()
            .map(|(dir, vs)| {
                vs.iter()
                    .map(|t| {
                        if dir == Dir::Up || dir == Dir::Down {
                            (t.row, t.col)
                        } else {
                            (t.col, t.row)
                        }
                    })
                    .into_group_map()
                    .into_values()
                    .map(|vs| {
                        vs.into_iter()
                            .sorted()
                            .tuple_windows()
                            .map(|(v0, v1)| if v1 > v0 + 1 { 1 } else { 0 })
                            .sum::<usize>()
                            + 1
                    })
                    .sum::<usize>()
            })
            .sum()
    }
}

impl Plot {
    fn new(tile: Tile) -> Self {
        Self {
            tile,
            borders: Vec::new(),
        }
    }
    fn add_border(&mut self, dir: Dir) {
        self.borders.push(dir);
    }
}

impl Grid {
    fn price(&mut self, bulk: bool) -> usize {
        let regions = self.scan();
        regions.iter().map(|r| r.price(bulk)).sum()
    }
    fn scan(&mut self) -> Vec<Region> {
        let mut scanned = HashSet::<Tile>::new();
        let mut regions: Vec<Region> = vec![];
        for r in 0..self.rows {
            for c in 0..self.cols {
                let t = self.get(r, c).unwrap();
                if scanned.contains(&t) {
                    continue;
                }
                let region = self.flood_region(t);
                for plot in region.plots.iter() {
                    scanned.insert(plot.tile);
                }
                regions.push(region);
            }
        }
        regions
    }
    fn flood_region(&self, tile: Tile) -> Region {
        let mut region = Region::default();
        let mut tiles = VecDeque::from([tile]);
        while !tiles.is_empty() {
            let tile = tiles.pop_front().unwrap();
            if region.contains(&tile) {
                continue;
            }
            let mut plot = Plot::new(tile);
            let (r, c) = (tile.row, tile.col);
            let nexts = [
                (Dir::Right, 0, 1),
                (Dir::Left, 0, -1),
                (Dir::Down, 1, 0),
                (Dir::Up, -1, 0),
            ];
            for next in nexts {
                let (dir, r, c) = (next.0, r + next.1, c + next.2);
                let next = self.get(r, c);
                match next {
                    Some(next) if next.ch == tile.ch => tiles.push_back(next),
                    Some(_) => plot.add_border(dir),
                    None => plot.add_border(dir),
                }
            }
            region.add(plot);
        }
        region
    }
    fn get(&self, r: i32, c: i32) -> Option<Tile> {
        (r >= 0 && c >= 0)
            .then_some((r as usize, c as usize))
            .and_then(|(r, c)| self.tiles.get(r)?.get(c).copied())
    }
    fn parse(s: &str) -> Self {
        let tiles: Vec<_> = s
            .trim()
            .lines()
            .enumerate()
            .map(|(row, l)| {
                l.chars()
                    .enumerate()
                    .map(|(col, ch)| {
                        let (row, col) = (row as i32, col as i32);
                        Tile { row, col, ch }
                    })
                    .collect_vec()
            })
            .collect();
        let (rows, cols) = (tiles.len() as i32, tiles[0].len() as i32);
        Self { tiles, rows, cols }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(eval(data::EX1, true), 140);
        assert_eq!(eval(data::EX2, true), 772);
        assert_eq!(eval(data::EX3, true), 1930);
    }

    #[test]
    fn ex2() {
        assert_eq!(eval(data::EX1, false), 80);
        assert_eq!(eval(data::EX2, false), 436);
        assert_eq!(eval(data::EX4, false), 236);
        assert_eq!(eval(data::EX5, false), 368);
        assert_eq!(eval(data::EX3, false), 1206);
    }

    #[test]
    fn pt() {
        assert_eq!(eval(data::IN1, true), 1396298);
        assert_eq!(eval(data::IN1, false), 853588);
    }
}

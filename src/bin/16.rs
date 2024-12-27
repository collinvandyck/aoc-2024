use itertools::Itertools;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
};
use strum::IntoEnumIterator;

#[allow(unused)]
mod data {
    pub static EX1: &str = include_str!("../../data/16/ex1");
    pub static IN1: &str = include_str!("../../data/16/in1");
}

fn main() {
    println!("pt1: {}", aoc_2024::timed(|| eval(data::IN1, true)));
    println!("pt2: {}", aoc_2024::timed(|| eval(data::IN1, false)));
}

fn eval(s: &str, pt1: bool) -> usize {
    let g = Grid::parse(s);
    g.score(pt1)
}

#[derive(Clone)]
struct Grid {
    tiles: Vec<Vec<Tile>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum_macros::EnumIter)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    fn delta(&self) -> Pt {
        match self {
            Dir::North => Pt(-1, 0),
            Dir::East => Pt(0, 1),
            Dir::South => Pt(1, 0),
            Dir::West => Pt(0, -1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pt(i32, i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Tile {
    pt: Pt,
    ch: char,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Deer {
    pt: Pt,
    dir: Dir,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Trail {
    score: usize,
    steps: Vec<Deer>,
}

impl Trail {
    fn deer(&self) -> Deer {
        self.steps.last().copied().unwrap()
    }
}

impl Grid {
    fn score(&self, pt1: bool) -> usize {
        let mut deers = HashMap::<Deer, usize>::new();
        let mut bests = Vec::<Trail>::new();
        let (start, end) = (self.find_ch('S'), self.find_ch('E'));
        let deer = Deer {
            pt: start.pt,
            dir: Dir::East,
        };
        let trail = Trail {
            score: 0,
            steps: vec![deer],
        };
        let mut trails = VecDeque::from([trail]);
        while !trails.is_empty() {
            let trail = trails.pop_front().unwrap();
            let deer = trail.deer();
            // if we are at the end, record the score
            if deer.pt == end.pt {
                if bests.is_empty() {
                    bests.push(trail);
                    continue;
                }
                let last = bests.last().unwrap();
                match last.score.cmp(&trail.score) {
                    Ordering::Equal => bests.push(trail),
                    Ordering::Greater => {
                        bests.clear();
                        bests.push(trail);
                    }
                    _ => {}
                }
                continue;
            }
            // otherwise, explore other options.
            for dir in Dir::iter() {
                let dlt = dir.delta();
                let next = self.get(deer.pt.0 + dlt.0, deer.pt.1 + dlt.1);
                if next.ch == '#' {
                    continue;
                }
                let score = trail.score + if dir == deer.dir { 1 } else { 1001 };
                let new_deer = Deer { pt: next.pt, dir };

                // if we have already been here but with a lower score, abort.
                let prev_score = deers.entry(new_deer).or_insert(score);
                if *prev_score < score {
                    // no need to keep going
                    continue;
                }
                *prev_score = score;

                let mut trail = trail.clone();
                trail.steps.push(new_deer);
                trail.score = score;
                trails.push_back(trail);
            }
        }
        if pt1 {
            bests.last().map(|t| t.score).unwrap_or_default()
        } else {
            let mut pts = HashSet::<Pt>::new();
            for deer in bests.iter().flat_map(|t| t.steps.iter()) {
                pts.insert(deer.pt);
            }
            pts.len()
        }
    }
    fn get(&self, row: i32, col: i32) -> Tile {
        let row = &self.tiles[row as usize];
        row[col as usize]
    }
    fn find_ch(&self, ch: char) -> Tile {
        let t = self.flat().filter(|t| t.ch == ch);
        t.copied().next().unwrap()
    }
    fn flat(&self) -> impl Iterator<Item = &Tile> {
        self.tiles.iter().flatten()
    }
    fn parse(s: &str) -> Self {
        let s = s.trim().lines().enumerate();
        let tiles = s
            .map(move |(row, l)| {
                let l = l.chars().enumerate();
                l.map(move |(col, ch)| {
                    Tile {
                        pt: Pt(row as i32, col as i32),
                        ch,
                    }
                })
                .collect()
            })
            .collect();
        Self { tiles }
    }
}

impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.tiles.iter();
        let s = s.map(|r| r.iter().map(|t| t.ch).collect::<String>());
        let s = s.collect_vec().join("\n");
        write!(f, "{s}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(eval(data::EX1, true), 7036);
    }

    #[test]
    fn ex2() {
        assert_eq!(eval(data::EX1, false), 45);
    }

    #[test]
    fn pt1() {
        assert_eq!(eval(data::IN1, true), 99460);
    }

    #[test]
    fn pt2() {
        assert_eq!(eval(data::IN1, false), 500);
    }
}

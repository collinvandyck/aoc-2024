use std::collections::HashSet;

#[cfg(test)]
static EX1: &str = include_str!("../../data/10/ex1");
#[cfg(test)]
static EX2: &str = include_str!("../../data/10/ex2");
static IN1: &str = include_str!("../../data/10/in1");

fn main() {
    println!("pt1: {}", aoc_2024::timed(|| eval(IN1, true)));
    println!("pt2: {}", aoc_2024::timed(|| eval(IN1, false)));
}

fn eval(s: &str, pt1: bool) -> usize {
    Grid::parse(s).scores(pt1)
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

type TileMem = HashSet<Tile>;
type Trail = Vec<Tile>;
type TrailMem = HashSet<Trail>;

impl Grid {
    fn scores(&self, pt1: bool) -> usize {
        let tiles = self.tiles.iter().flatten().copied();
        let trailheads = tiles.filter(|t| t.val == 0);
        trailheads
            .map(|t| {
                if pt1 {
                    self.trailhead_score(t)
                } else {
                    self.trailhead_rating(t)
                }
            })
            .sum()
    }
    fn trailhead_rating(&self, t: Tile) -> usize {
        fn traverse(g: &Grid, t: Tile, mut trail: Trail, trails: &mut TrailMem) {
            trail.push(t);
            if t.val == 9 {
                trails.insert(trail);
                return;
            }
            let (r, c) = (t.row, t.col);
            let nexts = [(r, c - 1), (r, c + 1), (r - 1, c), (r + 1, c)];
            nexts
                .into_iter()
                .filter_map(|(r, c)| g.get(r, c))
                .filter(|n| n.val == t.val + 1)
                .for_each(|t| traverse(g, t, trail.clone(), trails));
        }
        let trail = Trail::default();
        let mut trails = TrailMem::default();
        traverse(self, t, trail, &mut trails);
        trails.len()
    }
    fn trailhead_score(&self, t: Tile) -> usize {
        fn traverse(g: &Grid, t: Tile, hist: &mut TileMem) -> usize {
            if !hist.insert(t) {
                return 0;
            }
            if t.val == 9 {
                return 1;
            }
            let (r, c) = (t.row, t.col);
            let nexts = [(r, c - 1), (r, c + 1), (r - 1, c), (r + 1, c)];
            nexts
                .into_iter()
                .filter_map(|(r, c)| g.get(r, c))
                .filter(|n| n.val == t.val + 1)
                .map(|t| traverse(g, t, hist))
                .sum()
        }
        let mut mem = TileMem::default();
        traverse(self, t, &mut mem)
    }
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
        assert_eq!(eval(EX1, true), 1);
        assert_eq!(eval(EX2, true), 36);
        assert_eq!(eval(EX2, false), 81);
    }

    #[test]
    fn pts() {
        assert_eq!(eval(IN1, true), 820);
        assert_eq!(eval(IN1, false), 1786);
    }
}

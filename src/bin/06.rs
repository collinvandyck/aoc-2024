use aoc_2024 as aoc;
use std::collections::HashMap;

fn main() {
    let in1 = include_str!("../../data/06/in1");
    println!("pt1: {}", aoc::timed(|| eval(in1, true)));
    println!("pt2: {}", aoc::timed(|| eval(in1, false)));
}

fn eval(s: &str, pt1: bool) -> usize {
    let grid = Grid::from_str(s);
    grid.run(pt1)
}

#[derive(Clone)]
struct Grid {
    tiles: Vec<Vec<Tile>>,
    start: Tile,
    dir: Dir,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Tile(usize, usize, char);

static DIRS: &str = "URDL";

#[derive(Clone, Copy, Hash, Debug, PartialEq)]
struct Dir(usize, char);

impl Dir {
    fn rotate(self) -> Self {
        let pos = (self.0 + 1) % DIRS.len();
        Self(pos, DIRS.chars().nth(pos).unwrap())
    }
}

impl Grid {
    fn run(&self, pt1: bool) -> usize {
        let mut pt2 = 0;
        let (mut cur, mut dir) = (self.start, self.dir);
        let mut visited: HashMap<Tile, Vec<Dir>> = HashMap::new();
        loop {
            let tile_dirs = visited.entry(cur).or_default();
            if tile_dirs.contains(&dir) {
                break;
            }
            tile_dirs.push(dir);
            let Some(next) = self.next_tile(cur, dir) else { break };
            if next.2 == '#' {
                dir = dir.rotate();
                continue;
            }
            if !pt1 && !visited.contains_key(&next) {
                let (mut cur, mut dir, mut visited) = (cur, dir, visited.clone());
                let g2 = self.clone().replace(next, '#');
                visited.remove_entry(&cur);
                loop {
                    let tile_dirs = visited.entry(cur).or_default();
                    if tile_dirs.contains(&dir) {
                        pt2 += 1;
                        break;
                    }
                    tile_dirs.push(dir);
                    let Some(next) = g2.next_tile(cur, dir) else { break };
                    if next.2 == '#' {
                        dir = dir.rotate();
                        continue;
                    }
                    cur = next;
                }
            }
            cur = next;
        }
        if pt1 { visited.len() } else { pt2 }
    }
    fn next_tile(&self, tile: Tile, dir: Dir) -> Option<Tile> {
        let (rows, cols) = self.rows_cols();
        let (r, c) = match dir.1 {
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
    fn replace(mut self, tile: Tile, ch: char) -> Self {
        if let Some(row) = self.tiles.get_mut(tile.0) {
            if let Some(t) = row.get_mut(tile.1) {
                t.2 = ch;
            }
        }
        self
    }
    fn from_str(s: &str) -> Self {
        let mut start: Option<Tile> = None;
        let tiles = s
            .trim()
            .lines()
            .enumerate()
            .map(|(row, l)| {
                l.chars()
                    .enumerate()
                    .map(|(col, ch)| {
                        let tile = Tile(row, col, ch);
                        if ch == '^' {
                            start.replace(tile);
                        }
                        tile
                    })
                    .collect()
            })
            .collect();
        let dir = Dir(DIRS.chars().position(|p| p == 'U').unwrap(), 'U');
        let start = start.unwrap();
        Self { tiles, start, dir }
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
    fn pt2() {
        let s = include_str!("../../data/06/in1");
        assert_eq!(eval(s, false), 1976);
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

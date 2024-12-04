use aoc_2024 as aoc;
use itertools::iproduct;
use std::iter::successors;

fn main() {
    let in1 = include_str!("../../data/04/in1");
    println!("pt1: {}", aoc::timed(|| eval(in1, true)));
    println!("pt2: {}", aoc::timed(|| eval(in1, false)));
}

fn eval(s: &str, pt1: bool) -> usize {
    let board = Board::from_str(s);
    if pt1 { board.find_xmas() } else { board.find_tree() }
}

struct Board {
    tiles: Vec<Vec<Tile>>,
    rows: usize,
    cols: usize,
}

#[derive(Clone, Copy, Debug)]
struct Tile {
    ch: char,
    row: usize,
    col: usize,
}

impl Board {
    fn from_str(s: &str) -> Self {
        let tiles: Vec<Vec<_>> = s
            .trim()
            .lines()
            .enumerate()
            .map(|(row, s)| {
                s.trim()
                    .chars()
                    .enumerate()
                    .map(|(col, ch)| Tile { row, col, ch })
                    .collect()
            })
            .collect();
        let rows = tiles.len();
        let cols = tiles.first().map(|r| r.len()).unwrap();
        Self { tiles, rows, cols }
    }
    fn next(&self, t: &Tile, dr: isize, dc: isize) -> Option<&Tile> {
        let &Tile { row, col, .. } = t;
        let r = adjust(row, dr, self.rows - 1);
        let c = adjust(col, dc, self.cols - 1);
        r.and_then(|r| c.map(|c| (r, c)))
            .and_then(|(r, c)| self.get(r, c))
    }
    fn get(&self, row: usize, col: usize) -> Option<&Tile> {
        self.tiles.get(row).and_then(|r| r.get(col))
    }
    fn tiles_with(&self, ch: char) -> impl Iterator<Item = &Tile> {
        self.tiles
            .iter()
            .flatten()
            .filter(move |t| t.ch == ch)
    }
    fn find_tree(&self) -> usize {
        self.tiles_with('A')
            .filter(|t| {
                let vs = iproduct!([1, -1], [1, -1])
                    .flat_map(|(r, c)| self.next(t, r, c).map(|t| t.ch))
                    .collect::<String>();
                matches!(vs.as_str(), "MMSS" | "MSMS" | "SSMM" | "SMSM")
            })
            .count()
    }
    fn find_xmas(&self) -> usize {
        self.tiles_with('X')
            .flat_map(|t| {
                iproduct!(-1..=1, -1..=1)
                    .filter(|&x| x != (0, 0))
                    .map(move |(r, c)| successors(Some(t), move |t| self.next(t, r, c)).take(4))
                    .map(|i| i.map(|t| t.ch).collect::<String>())
            })
            .filter(|s| s == "XMAS")
            .count()
    }
}

fn adjust(val: usize, amt: isize, max: usize) -> Option<usize> {
    (val as isize)
        .checked_add(amt)
        .filter(|&x| x >= 0 && x <= max as isize)
        .map(|x| x as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex01() {
        let s = include_str!("../../data/04/ex1");
        assert_eq!(eval(s, true), 18);
    }

    #[test]
    fn ex02() {
        let s = include_str!("../../data/04/ex1");
        assert_eq!(eval(s, false), 9);
    }

    #[test]
    fn pt1() {
        let s = include_str!("../../data/04/in1");
        assert_eq!(eval(s, true), 2458);
    }

    #[test]
    fn pt2() {
        let s = include_str!("../../data/04/in1");
        assert_eq!(eval(s, false), 1945);
    }
}

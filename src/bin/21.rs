#![allow(unused)]

use core::panic;
use itertools::Itertools;
use std::{
    collections::{HashMap, hash_map},
    fmt::Display,
    iter, usize,
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[allow(unused)]
mod data {
    pub static EX1: &str = include_str!("../../data/21/ex1");
    pub static IN1: &str = include_str!("../../data/21/in1");
}

fn main() {
    println!("pt1: {}", aoc_2024::timed(|| eval(data::IN1)));
}

/// We want to start with the path that should be traveled on the numeric keypad. The robot assigned
/// to the numeric keypad starts with the arm pointed at A. For each input in the specified code,
/// this corresponds to a shortest movement on the numeric keypad.
///
/// The robot assigned to the numeric keypad is controlled by a directional keypad.
/// That robot is controlled by a directional keypad.
/// That robot is controlled by a directional keypad.
///
/// DK = Directional Keypad
/// NK = Numeric Keypad
///
/// DK1 (me) -> DK2 (robot) -> DK3 (robot) -> NK
///
/// For each movement on NK, this needs to be translated into movement on the DK3, which needs to be
/// translated into movement on DK2, and then DK1.
///
/// There can be many shortest paths at any particular stage of the problem, but not all shortest
/// paths result in shortest overall keypresses overall. The sequence taken for one shortest path
/// might result in more keypresses in an outer layer.
///
///
/// +---+---+---+
/// | 7 | 8 | 9 |
/// +---+---+---+
/// | 4 | 5 | 6 |
/// +---+---+---+
/// | 1 | 2 | 3 |
/// +---+---+---+
///     | 0 | A |
///     +---+---+
///
///     +---+---+
///     | ^ | A |
/// +---+---+---+
/// | < | v | > |
/// +---+---+---+
fn eval(s: &str) -> usize {
    let codes = parse(s);
    // 48
    codes
        .iter()
        .map(|code| {
            let mut dk1 = Keypad::directional();
            let mut dk2 = Keypad::directional();
            let mut nk1 = Keypad::numeric();
            println!("{code}");
            let dirs = nk1.move_all(code.chars());
            let dirs = to_keypresses(&dirs);
            println!("{}", dirs.to_string());
            let dirs = dk1.move_all(dirs.into_iter());
            let dirs = to_keypresses(&dirs);
            println!("{}", dirs.to_string());
            let dirs = dk2.move_all(dirs.into_iter());
            let dirs = to_keypresses(&dirs);
            println!("{}", dirs.to_string());
            let len = dirs.len();
            let code = code.numeric();
            let res = len * code;
            println!("{len} x {code} = {res}");
            res
        })
        .sum()
}

fn to_keypresses(dirs: &[Vec<Dir>]) -> Vec<char> {
    dirs.into_iter()
        .flat_map(|dirs| {
            dirs.into_iter()
                .map(|d| d.ch())
                .chain(iter::once('A'))
        })
        .collect_vec()
}

#[derive(Debug)]
struct Keypad {
    graph: Graph,
    pos: Tile,
}

impl Keypad {
    fn numeric() -> Self {
        let graph = Graph::parse("789\n456\n123\nX0A");
        let pos = graph.get_ch('A');
        Self { graph, pos }
    }
    fn directional() -> Self {
        let graph = Graph::parse("X^A\n<v>");
        let pos = graph.get_ch('A');
        Self { graph, pos }
    }
    fn move_all(&mut self, chs: impl IntoIterator<Item = char>) -> Vec<Vec<Dir>> {
        chs.into_iter()
            .map(|ch| self.move_to(ch))
            .collect()
    }
    fn move_to(&mut self, ch: char) -> Vec<Dir> {
        let dst = self.graph.get_ch(ch);
        let path: &[Tile] = self.graph.get_path(self.pos, dst);
        self.pos = dst;
        path_to_dirs(path)
    }
}

fn path_to_dirs(tiles: &[Tile]) -> Vec<Dir> {
    tiles
        .iter()
        .tuple_windows()
        .map(|(t1, t2)| t1.dir_to(t2))
        .collect()
}

#[derive(Debug)]
struct Code(Vec<char>);

impl Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().collect::<String>())
    }
}

impl Code {
    fn chars(&self) -> impl Iterator<Item = char> + '_ {
        self.0.iter().copied()
    }
    fn numeric(&self) -> usize {
        let mut chs = self.0.iter();
        let chs = chs.skip_while(|&&ch| ch == '0');
        let chs = chs.take_while(|&&ch| ch.is_numeric());
        let s = chs.collect::<String>();
        s.parse::<usize>().unwrap()
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn delta(&self) -> Pt {
        match self {
            Self::Up => Pt(-1, 0),
            Self::Down => Pt(1, 0),
            Self::Left => Pt(0, -1),
            Self::Right => Pt(0, 1),
        }
    }
    fn ch(&self) -> char {
        match self {
            Self::Up => '^',
            Self::Down => 'v',
            Self::Left => '<',
            Self::Right => '>',
        }
    }
}

trait DisplayExt {
    fn to_string(&self) -> String;
}

impl DisplayExt for Vec<char> {
    fn to_string(&self) -> String {
        self.iter().collect()
    }
}

trait TileExt {
    fn tile_chars(&self) -> Vec<char>;
}

impl TileExt for Vec<Tile> {
    fn tile_chars(&self) -> Vec<char> {
        self.iter().map(|t| t.ch).collect_vec()
    }
}

#[derive(Debug)]
struct Graph {
    rows: usize,
    cols: usize,
    tiles: Vec<Tile>,
    paths: HashMap<(Tile, Tile), Vec<Tile>>,
}

impl Graph {
    fn get_path(&mut self, start: Tile, end: Tile) -> &[Tile] {
        let key = (start, end);
        let entry = self.paths.entry(key);
        if matches!(entry, hash_map::Entry::Vacant(..)) {
            fn dfs(g: &Graph, end: &Tile, stack: &mut Vec<Tile>) -> Option<Vec<Tile>> {
                let cur = stack.last().copied().unwrap();
                if cur.ch == end.ch {
                    return Some(stack.clone());
                }
                let nexts = [
                    //
                    Dir::Up,
                    Dir::Right,
                    Dir::Down,
                    Dir::Left,
                ];
                let nexts = nexts.into_iter().map(|d| d.delta());
                let nexts = nexts.flat_map(|d| g.next(cur, d));
                let nexts = nexts.filter(|t| !stack.contains(t));
                let nexts = nexts.collect_vec();
                let mut best = Option::<Vec<Tile>>::None;
                for next in nexts {
                    assert!(!next.void);
                    stack.push(next);
                    if let Some(res) = dfs(g, end, stack) {
                        best = match best {
                            Some(b) if b.len() <= res.len() => Some(b),
                            _ => Some(res),
                        }
                    }
                    stack.pop();
                }
                best
            }
            let mut stack = vec![start];
            let mut path = dfs(self, &end, &mut stack).unwrap();
            self.paths.insert(key, path.clone());
        }
        self.paths.get(&key).unwrap()
    }
    fn next(&self, tile: Tile, dlt: Pt) -> Option<Tile> {
        let (row, col) = (tile.pt.0 + dlt.0, tile.pt.1 + dlt.1);
        if row < 0 || col < 0 || row >= self.rows as i32 || col >= self.cols as i32 {
            return None;
        }
        let (row, col) = (row as usize, col as usize);
        let idx = row * self.cols + col;
        self.tiles.get(idx).filter(|t| !t.void).copied()
    }
    fn get_ch(&self, ch: char) -> Tile {
        let t = self.tiles.iter().find(|t| t.ch == ch);
        match t.copied() {
            Some(v) => v,
            None => {
                panic!("no ch '{ch}' found");
            }
        }
    }
    fn parse(s: &str) -> Graph {
        let rows = s.trim().lines().enumerate();
        let tiles: Vec<Vec<Tile>> = rows
            .map(|(row, l)| {
                l.chars()
                    .enumerate()
                    .map(|(col, ch)| {
                        let void = ch == 'X';
                        let (row, col) = (row as i32, col as i32);
                        let pt = Pt(row, col);
                        Tile { pt, ch, void }
                    })
                    .collect_vec()
            })
            .collect_vec();
        let rows = tiles.len();
        let cols = tiles[0].len();
        let paths = HashMap::new();
        Self {
            rows,
            cols,
            tiles: tiles.into_iter().flatten().collect(),
            paths,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Tile {
    pt: Pt,
    ch: char,
    void: bool,
}

impl Tile {
    fn dir_to(&self, other: &Tile) -> Dir {
        if self.pt.0.abs_diff(other.pt.0) == 0 {
            return if self.pt.1 < other.pt.1 { Dir::Right } else { Dir::Left };
        }
        if self.pt.1.abs_diff(other.pt.1) == 0 {
            return if self.pt.0 < other.pt.0 { Dir::Down } else { Dir::Up };
        }
        unreachable!()
    }
    fn is_void(&self) -> bool {
        self.void
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Pt(i32, i32);

fn parse(s: &str) -> Vec<Code> {
    s.trim()
        .lines()
        .map(|l| l.chars().collect())
        .map(Code)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex_first_code() {
        let s = data::EX1
            .trim()
            .lines()
            .take(1)
            .collect::<String>();
        assert_eq!(eval(&s), 1972);
    }

    //#[test]
    //fn test_path_ex1() {
    //assert_eq!(eval(data::EX1), 126384);
    //}
}

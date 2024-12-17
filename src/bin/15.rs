use core::panic;
use itertools::Itertools;
use std::char;

#[allow(unused)]
mod data {
    pub static EX1: &str = include_str!("../../data/15/ex1");
    pub static IN1: &str = include_str!("../../data/15/in1");
}

fn main() {
    println!("pt1: {}", aoc_2024::timed(|| eval(data::IN1, true)));
    println!("pt2: {}", aoc_2024::timed(|| eval(data::IN1, false)));
}

fn eval(s: &str, pt1: bool) -> usize {
    let mut game = parse(s);
    if !pt1 {
        game = game.inflate();
    }
    game.run()
}

struct Game {
    grid: Grid,
    moves: Vec<Move>,
}

#[derive(Clone)]
struct Grid {
    tiles: Vec<Vec<Tile>>,
    robot: Tile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pt(i32, i32);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Tile {
    pt: Pt,
    ch: char,
}

#[derive(Debug, Clone, Copy)]
struct Move(char);

impl Game {
    fn run(mut self) -> usize {
        println!("{:?}\n", self.grid);
        let moves = self.moves.iter().copied();
        for mv in moves {
            self.grid.accept(mv);
        }
        println!("{:?}\n", self.grid);
        self.grid.score()
    }
}

impl Grid {
    fn accept(&mut self, mv: Move) {
        let delta = mv.to_delta();
        if self.move_tile(self.robot, delta) {
            self.robot.pt.0 += delta.0;
            self.robot.pt.1 += delta.1;
        }
    }
    // recursive. return true if tile was moved
    fn move_tile(&mut self, tile: Tile, delta: (i32, i32)) -> bool {
        let next = self.next(tile, delta);
        if next.is_wall() {
            return false;
        }
        if next.is_wide() && delta.0 != 0 {
            let backup = self.clone();
            let pair: (Tile, Tile) = self.pair(next);
            if !self.move_pair(pair, delta) {
                *self = backup; // restore
                return false;
            }
        } else {
            if !next.is_empty() {
                if !self.move_tile(next, delta) {
                    return false;
                }
            }
        }
        self.set_ch(next, tile.ch);
        self.set_ch(tile, '.');
        true
    }
    fn move_pair(&mut self, pair: (Tile, Tile), delta: (i32, i32)) -> bool {
        let (n1, n2) = (self.next(pair.0, delta), (self.next(pair.1, delta)));
        let ok = match (n1.ch, n2.ch) {
            ('#', _) | (_, '#') => false,
            ('[', ']') => self.move_pair((n1, n2), delta),
            (']', '.') => self.move_pair(self.pair(n1), delta),
            ('.', '[') => self.move_pair(self.pair(n2), delta),
            (']', '[') => self.move_pair(self.pair(n1), delta) && self.move_pair(self.pair(n2), delta),
            ('.', '.') => true,
            _ => {
                panic!("bad n chs: {n1:?} {n2:?}");
            }
        };
        if ok {
            self.set_ch(n1, pair.0.ch);
            self.set_ch(n2, pair.1.ch);
            self.set_ch(pair.0, '.');
            self.set_ch(pair.1, '.');
        }
        ok
    }
    fn score(&self) -> usize {
        let boxes = self.flat().filter(|t| t.ch == 'O' || t.ch == '[');
        let scores = boxes.map(|b| (100 * b.pt.0 + b.pt.1));
        scores.sum::<i32>() as usize
    }
    fn next(&self, tile: Tile, delta: (i32, i32)) -> Tile {
        self.get(tile.pt.0 + delta.0, tile.pt.1 + delta.1)
    }
    fn flat(&self) -> impl Iterator<Item = Tile> + '_ {
        self.tiles.iter().flatten().copied()
    }
    fn get(&self, row: i32, col: i32) -> Tile {
        let row = &self.tiles[row as usize];
        row[col as usize]
    }
    fn set_ch(&mut self, tile: Tile, ch: char) {
        let row = self.tiles.get_mut(tile.pt.0 as usize).unwrap();
        let t = row.get_mut(tile.pt.1 as usize).unwrap();
        t.ch = ch;
    }
    fn pair(&self, tile: Tile) -> (Tile, Tile) {
        match tile.ch {
            '[' => (tile, self.get(tile.pt.0, tile.pt.1 + 1)),
            ']' => (self.get(tile.pt.0, tile.pt.1 - 1), tile),
            _ => panic!("bad ch: {}", tile.ch),
        }
    }
    fn parse(s: &str) -> Self {
        let tiles: Vec<_> = s
            .trim()
            .lines()
            .enumerate()
            .map(|(row, l)| {
                l.trim()
                    .chars()
                    .enumerate()
                    .map(|(col, ch)| {
                        let (row, col) = (row as i32, col as i32);
                        let pt = Pt(row, col);
                        Tile { pt, ch }
                    })
                    .collect_vec()
            })
            .collect();
        let robot = find_robot(&tiles);
        Self { tiles, robot }
    }
}

impl Game {
    fn inflate(mut self) -> Self {
        let rows = self.grid.tiles.iter();
        let rows = rows
            .map(|row| {
                let row = row.iter().copied();
                let row = row.map(|mut tile| {
                    tile.pt.1 *= 2;
                    let mut next = tile;
                    next.pt.1 += 1;
                    if tile.ch == 'O' {
                        tile.ch = '[';
                        next.ch = ']';
                    }
                    if tile.ch == '@' {
                        next.ch = '.'
                    }
                    [tile, next]
                });
                row.flatten().collect()
            })
            .collect();
        self.grid.tiles = rows;
        self.grid.robot = find_robot(&self.grid.tiles);
        self
    }
}
impl Tile {
    fn is_box(&self) -> bool {
        self.ch == 'O' || self.is_wide()
    }
    fn is_wide(&self) -> bool {
        self.ch == '[' || self.ch == ']'
    }
    fn is_wall(&self) -> bool {
        self.ch == '#'
    }
    fn is_empty(&self) -> bool {
        !self.is_box() && !self.is_wall()
    }
}

impl Move {
    fn to_delta(&self) -> (i32, i32) {
        match self.0 {
            '<' => (0, -1),
            '>' => (0, 1),
            '^' => (-1, 0),
            'v' => (1, 0),
            _ => unreachable!(),
        }
    }
}

fn find_robot(tiles: &[Vec<Tile>]) -> Tile {
    let mut tiles = tiles.iter().flatten();
    let robot = tiles.find(|t| t.ch == '@');
    robot.copied().unwrap()
}

fn parse(s: &str) -> Game {
    let (grid, moves) = s.trim().split_once("\n\n").unwrap();
    let grid = Grid::parse(grid);
    let moves = moves
        .lines()
        .flat_map(str::chars)
        .map(Move)
        .collect_vec();
    Game { grid, moves }
}

impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .tiles
            .iter()
            .map(|row| row.iter().map(|t| t.ch).collect::<String>())
            .collect_vec()
            .join("\n");
        write!(f, "{s}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(eval(data::EX1, true), 10092);
    }

    #[test]
    fn ex2() {
        assert_eq!(eval(data::EX1, false), 9021);
    }

    #[test]
    fn pt1() {
        assert_eq!(eval(data::IN1, true), 1475249);
    }

    #[test]
    fn pt2() {
        assert_eq!(eval(data::IN1, false), 1509724);
    }
}

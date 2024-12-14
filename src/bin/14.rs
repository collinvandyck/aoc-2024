use itertools::Itertools;
use std::iter::repeat_n;

#[allow(unused)]
mod data {
    pub static EX1: &str = include_str!("../../data/14/ex1");
    pub static IN1: &str = include_str!("../../data/14/in1");
}

fn main() {
    println!("pt1: {}", aoc_2024::timed(|| eval(data::IN1, true)));
    println!("pt2: {}", aoc_2024::timed(|| eval(data::IN1, false)));
}

fn eval(s: &str, pt1: bool) -> usize {
    let mut game = parse(s);
    if pt1 { game.safety_factor() } else { game.find_tree() }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Pos(i32, i32);

#[derive(Debug, Clone, Copy, Hash)]
struct Vel(i32, i32);

#[derive(Debug, Clone, Copy, Hash)]
struct Robot {
    pos: Pos,
    vel: Vel,
}

impl Robot {
    fn new(pos: Pos, vel: Vel) -> Self {
        Self { pos, vel }
    }
}

#[derive(Debug, Clone)]
struct Game {
    robots: Vec<Robot>,
    rows: i32,
    cols: i32,
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut grid: Vec<Vec<char>> = (0..self.rows as usize)
            .map(|_| repeat_n('.', self.cols as usize).collect_vec())
            .collect_vec();
        for r in &self.robots {
            let Pos(r, c) = r.pos;
            let (r, c) = (r as usize, c as usize);
            grid[r][c] = '#';
        }
        let s = grid
            .into_iter()
            .map(|r| r.into_iter().collect::<String>())
            .collect_vec()
            .join("\n");
        write!(f, "{s}")
    }
}

impl Game {
    fn find_tree(&mut self) -> usize {
        let mv = |p: i32, v: i32, bound: i32| {
            let p = (p + v) % bound;
            if p < 0 { bound + p } else { p }
        };
        (1..)
            .find(|_| {
                for r in &mut self.robots {
                    r.pos.0 = mv(r.pos.0, r.vel.0, self.rows);
                    r.pos.1 = mv(r.pos.1, r.vel.1, self.cols);
                }
                let min_run = 10;
                self.robots
                    .iter()
                    .map(|r| (r.pos.0, r.pos.1))
                    .into_group_map()
                    .values()
                    .filter(|vs| vs.len() >= min_run)
                    .any(|vs| {
                        let max = vs
                            .iter()
                            .sorted()
                            .fold((0, 0, None), |(max, run, last), &i| {
                                match last {
                                    None => (1, 1, Some(i)),
                                    Some(last) if i == last + 1 => (max.max(run + 1), run + 1, Some(i)),
                                    Some(_) => (max, 1, Some(i)),
                                }
                            });
                        max.0 >= min_run
                    })
            })
            .unwrap()
    }
    fn safety_factor(&self) -> usize {
        let secs = 100;
        let mv = |p: i32, v: i32, bound: i32| {
            let p = (p + v * secs) % bound;
            if p < 0 { bound + p } else { p }
        };
        let robots = self
            .robots
            .iter()
            .map(|r| {
                let mut r = *r;
                r.pos.0 = mv(r.pos.0, r.vel.0, self.rows);
                r.pos.1 = mv(r.pos.1, r.vel.1, self.cols);
                r
            })
            .collect_vec();
        let (rmid, cmid) = (self.rows / 2, self.cols / 2);
        let (ul, ur, bl, br) = robots.iter().fold((0, 0, 0, 0), |mut acc, r| {
            match r.pos {
                Pos(r, c) if r < rmid && c < cmid => acc.0 += 1,
                Pos(r, c) if r < rmid && c > cmid => acc.1 += 1,
                Pos(r, c) if r > rmid && c < cmid => acc.2 += 1,
                Pos(r, c) if r > rmid && c > cmid => acc.3 += 1,
                _ => {}
            }
            acc
        });
        ul * ur * bl * br
    }
    fn new(robots: Vec<Robot>, rows: i32, cols: i32) -> Self {
        Self { robots, rows, cols }
    }
}

fn parse(s: &str) -> Game {
    let robots = s
        .trim()
        .lines()
        .map(|s| s.split_once(" ").unwrap())
        .map(|(p, v)| {
            let (p1, p2) = &p[2..].split_once(",").unwrap();
            let (p1, p2) = (p1.parse().unwrap(), p2.parse().unwrap());
            let pos = Pos(p2, p1);
            let (v1, v2) = &v[2..].split_once(",").unwrap();
            let (v1, v2) = (v1.parse().unwrap(), v2.parse().unwrap());
            let vel = Vel(v2, v1);
            Robot::new(pos, vel)
        })
        .collect();
    let (rows, cols) = if s == data::EX1 { (7, 11) } else { (103, 101) };
    Game::new(robots, rows, cols)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exs() {
        assert_eq!(eval(data::EX1, true), 12);
    }

    #[test]
    fn pts() {
        assert_eq!(eval(data::IN1, true), 225521010);
        assert_eq!(eval(data::IN1, false), 7774);
    }
}

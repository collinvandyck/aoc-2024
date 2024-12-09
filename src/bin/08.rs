use itertools::Itertools;
use std::collections::HashSet;

#[cfg(test)]
static EX1: &str = include_str!("../../data/08/ex1");
static IN1: &str = include_str!("../../data/08/in1");

fn main() {
    println!("pt1: {}", aoc_2024::timed(|| eval(IN1, true)));
    println!("pt2: {}", aoc_2024::timed(|| eval(IN1, false)));
}

fn eval(s: &str, pt1: bool) -> usize {
    let g = Grid::parse(s);
    let mut antinodes: HashSet<Tile> = HashSet::new();
    for (_, attns) in g.anntenaes().into_group_map_by(|t| t.2) {
        for (a1, a2) in attns.clone().into_iter().tuple_combinations() {
            if !pt1 {
                antinodes.insert(a1);
                antinodes.insert(a2);
            }
            let slope = a1.slope(a2);
            {
                let mut a2 = a2;
                while let Some(t) = g.get(slope.0.apply(a2.0), slope.1.apply(a2.1)) {
                    antinodes.insert(t);
                    if pt1 {
                        break;
                    }
                    a2 = t;
                }
            }
            {
                let mut a1 = a1;
                let slope = Slope(slope.0.neg(), slope.1.neg());
                while let Some(t) = g.get(slope.0.apply(a1.0), slope.1.apply(a1.1)) {
                    antinodes.insert(t);
                    if pt1 {
                        break;
                    }
                    a1 = t;
                }
            }
        }
    }
    antinodes.len()
}

#[derive(Clone)]
struct Grid {
    tiles: Vec<Vec<Tile>>,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Tile(usize, usize, char);

impl Tile {
    fn slope(self, other: Tile) -> Slope {
        let dr = Delta::new(self.0, other.0);
        let dc = Delta::new(self.1, other.1);
        Slope(dr, dc)
    }
}

#[derive(Debug)]
struct Slope(Delta, Delta);

#[derive(Debug, Clone, Copy)]
enum Delta {
    Pos(usize),
    Neg(usize),
}

impl Delta {
    fn neg(self) -> Self {
        match self {
            Delta::Pos(d) => Delta::Neg(d),
            Delta::Neg(d) => Delta::Pos(d),
        }
    }
    fn apply(self, v: usize) -> usize {
        match self {
            Delta::Pos(d) => v + d,
            Delta::Neg(d) => v - d,
        }
    }
    fn new(n1: usize, n2: usize) -> Self {
        if n1 >= n2 {
            Delta::Neg(n1 - n2)
        } else {
            Delta::Pos(n2 - n1)
        }
    }
}

impl Grid {
    fn parse(s: &str) -> Self {
        let tiles = s
            .trim()
            .lines()
            .enumerate()
            .map(|(r, line)| {
                line.trim()
                    .chars()
                    .enumerate()
                    .map(|(c, ch)| Tile(r + line.len(), c + line.len(), ch))
                    .collect()
            })
            .collect();
        Self { tiles }
    }
    fn anntenaes(&self) -> impl Iterator<Item = Tile> + '_ {
        self.tiles.iter().flat_map(|row| {
            row.iter()
                .filter(|r| r.2.is_alphanumeric())
                .copied()
        })
    }
    fn get(&self, row: usize, col: usize) -> Option<Tile> {
        let dim = self.tiles[0].len();
        row.checked_sub(dim)
            .and_then(|row| col.checked_sub(dim).map(|col| (row, col)))
            .and_then(|(row, col)| self.tiles.get(row).and_then(|row| row.get(col)))
            .copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expt1() {
        assert_eq!(eval(EX1, true), 14);
    }

    #[test]
    fn expt2() {
        assert_eq!(eval(EX1, false), 34);
    }

    #[test]
    fn pt1() {
        assert_eq!(eval(IN1, true), 291);
    }

    #[test]
    fn pt2() {
        assert_eq!(eval(IN1, false), 1015);
    }
}

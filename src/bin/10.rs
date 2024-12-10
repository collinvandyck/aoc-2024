#![allow(unused)]

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
    0
}

struct Grid {
    vals: Vec<Vec<u8>>,
}

impl Grid {
    fn parse(s: &str) -> Self {
        let vals = s
            .trim()
            .lines()
            .map(|l| {
                l.chars()
                    .map(|ch| ch.to_digit(10).unwrap())
                    .map(|c| c as u8)
                    .collect()
            })
            .collect();
        Self { vals }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expt1() {
        assert_eq!(eval(EX1, true), 2);
    }
}

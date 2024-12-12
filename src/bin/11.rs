use std::collections::HashMap;

#[cfg(test)]
static EX1: &str = include_str!("../../data/11/ex1");
static IN1: &str = include_str!("../../data/11/in1");

fn main() {
    println!("pt1: {}", aoc_2024::timed(|| eval(IN1, true)));
    println!("pt2: {}", aoc_2024::timed(|| eval(IN1, false)));
}

fn eval(s: &str, pt1: bool) -> usize {
    let mut pile = Pile::new(s);
    pile.blink_n(if pt1 { 25 } else { 75 })
}

#[derive(Debug)]
struct Pile {
    vs: Vec<(usize, u64)>,
}

impl Pile {
    fn new(s: &str) -> Self {
        let vs = s
            .split_whitespace()
            .map(str::parse)
            .map(Result::unwrap)
            .map(|v| (1, v))
            .collect();
        Self { vs }
    }
    fn blink_n(&mut self, times: usize) -> usize {
        for _ in 0..times {
            self.blink();
        }
        self.count()
    }
    fn blink(&mut self) {
        for i in 0..self.vs.len() {
            let (count, v) = self.vs[i];
            if v == 0 {
                self.vs[i].1 = 1;
                continue;
            }
            if let Some((l, r)) = split(v) {
                self.vs[i].1 = l;
                self.vs.push((count, r));
                continue;
            }
            self.vs[i].1 *= 2024;
        }
        let mut counts = HashMap::new();
        for (count, v) in self.vs.drain(..) {
            *counts.entry(v).or_default() += count;
        }
        for (v, count) in counts {
            self.vs.push((count, v));
        }
    }
    fn count(&self) -> usize {
        self.vs.iter().map(|t| t.0).sum()
    }
}

fn split(v: u64) -> Option<(u64, u64)> {
    let nd = (v as f64).log10() as usize + 1;
    if nd % 2 != 0 {
        return None;
    }
    let p = 10_u32.pow(nd as u32 / 2) as u64;
    let l = v / p;
    let r = v % p;
    Some((l, r))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exs() {
        assert_eq!(eval(EX1, true), 55312);
    }

    #[test]
    fn pts() {
        assert_eq!(eval(IN1, true), 203457);
        assert_eq!(eval(IN1, false), 241394363462435);
    }
}

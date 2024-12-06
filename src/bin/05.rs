use aoc_2024 as aoc;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let in1 = include_str!("../../data/05/in1");
    println!("pt1: {}", aoc::timed(|| eval(in1, true)));
    println!("pt2: {}", aoc::timed(|| eval(in1, false)));
}

#[derive(Default, Debug)]
struct Tracker {
    befores: HashMap<usize, HashSet<usize>>,
}

impl Tracker {
    fn from_str(s: &str) -> Self {
        let mut tracker = Self::default();
        let tups = s.lines().map(|l| {
            l.split("|")
                .map(str::parse)
                .map(Result::unwrap)
                .collect_tuple()
                .unwrap()
        });
        for (before, after) in tups {
            tracker.add(before, after);
        }
        tracker
    }
    fn add(&mut self, before: usize, after: usize) {
        let befores = self.befores.entry(after).or_default();
        befores.insert(before);
    }
    fn has_before(&self, page: &[usize], val: usize, target: usize) -> bool {
        let mut visited = vec![];
        let mut queue = vec![val];
        while !queue.is_empty() {
            let val = queue.remove(0);
            if visited.contains(&val) {
                continue;
            }
            visited.push(val);
            let others = self
                .befores
                .get(&val)
                .into_iter()
                .flat_map(|f| f.iter().copied())
                .filter(|f| page.contains(f));
            for other in others {
                if other == target {
                    return true;
                }
                if !visited.contains(&other) {
                    queue.push(other);
                }
            }
        }
        false
    }
    // lol bubblesort
    fn sort(&self, mut page: Vec<usize>) -> Vec<usize> {
        for _ in 0..page.len() - 1 {
            for j in 0..page.len() - 1 {
                let val = page[j];
                let oth = page[j + 1];
                if self.has_before(&page, val, oth) {
                    page[j] = oth;
                    page[j + 1] = val;
                }
            }
        }
        page
    }
}

fn eval(s: &str, pt1: bool) -> usize {
    let parts = s.split("\n\n").collect_vec();
    let tracker = Tracker::from_str(parts[0]);
    let pages: Vec<Vec<usize>> = parts[1]
        .trim()
        .lines()
        .map(|l| {
            l.split(",")
                .map(str::parse)
                .map(Result::unwrap)
                .collect()
        })
        .collect_vec();
    pages
        .into_iter()
        .filter_map(|page| {
            for idx in 0..page.len() - 1 {
                for &after in &page[idx + 1..] {
                    if tracker.has_before(&page, page[idx], after) {
                        return (!pt1).then(|| tracker.sort(page));
                    }
                }
            }
            pt1.then_some(page)
        })
        .map(|v| v[v.len() / 2])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex01() {
        let s = include_str!("../../data/05/ex1");
        assert_eq!(eval(s, true), 143);
    }

    #[test]
    fn ex02() {
        let s = include_str!("../../data/05/ex1");
        assert_eq!(eval(s, false), 123);
    }

    #[test]
    fn pt1() {
        let s = include_str!("../../data/05/in1");
        assert_eq!(eval(s, true), 5248);
    }

    #[test]
    fn pt2() {
        let s = include_str!("../../data/05/in1");
        assert_eq!(eval(s, false), 4507);
    }
}

#![allow(unused)]

fn main() {
    println!("pt1: {}", run(include_str!("../../data/01/in1"), true));
    println!("pt2: {}", run(include_str!("../../data/01/in1"), false));
}

fn run(s: &str, pt1: bool) -> usize {
    let (mut v1, mut v2): (Vec<_>, Vec<_>) = s
        .trim()
        .lines()
        .map(|s| {
            let ps = s
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (ps[0], ps[1])
        })
        .unzip();
    v1.sort();
    v2.sort();
    if pt1 {
        return v1
            .into_iter()
            .zip(v2)
            .map(|(n1, n2)| n1.abs_diff(n2))
            .sum();
    }
    v1.into_iter()
        .map(|n1| n1 * v2.iter().filter(|n2| n1 == **n2).count())
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use crate::run;

    #[test]
    fn pt1() {
        let s = include_str!("../../data/01/in1");
        assert_eq!(run(s, true), 3569916);
    }

    #[test]
    fn pt2() {
        let s = include_str!("../../data/01/in1");
        assert_eq!(run(s, false), 26407426);
    }

    #[test]
    fn ex1() {
        let ex1 = include_str!("../../data/01/ex1");
        assert_eq!(run(ex1, true), 11);
    }

    #[test]
    fn ex2() {
        let ex1 = include_str!("../../data/01/ex1");
        assert_eq!(run(ex1, false), 31);
    }
}

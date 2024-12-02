fn main() {
    println!("pt1: {}", is_safe(include_str!("../../data/02/in1"), true));
    println!("pt2: {}", is_safe(include_str!("../../data/02/in1"), false));
}

fn is_safe(s: &str, pt1: bool) -> usize {
    s.trim()
        .lines()
        .map(|line| {
            line.trim()
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|line| {
            pt1.then_some(is_valid(line)).unwrap_or_else(|| {
                is_valid(line)
                    || (0..line.len()).any(|i| {
                        let s1 = &line[0..i];
                        let s2 = &line[i + 1..];
                        is_valid(s1.into_iter().chain(s2))
                    })
            })
        })
        .count()
}

fn is_valid<'a>(iter: impl IntoIterator<Item = &'a i64> + Clone) -> bool {
    let (pos, neg, dst) = iter
        .clone()
        .into_iter()
        .zip(iter.into_iter().skip(1))
        .map(|t| t.1 - t.0)
        .fold((true, true, true), |a, n| {
            (a.0 && n > 0, a.1 && n < 0, a.2 && (1..=3).contains(&n.abs()))
        });
    (pos || neg) && dst
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1() {
        let in1 = include_str!("../../data/02/in1");
        assert_eq!(is_safe(in1, true), 639);
    }

    #[test]
    fn pt2() {
        let in1 = include_str!("../../data/02/in1");
        assert_eq!(is_safe(in1, false), 674);
    }

    #[test]
    fn ex1() {
        let ex1 = include_str!("../../data/02/ex1");
        assert_eq!(is_safe(ex1, true), 2);
    }

    #[test]
    fn ex2() {
        let ex1 = include_str!("../../data/02/ex1");
        assert_eq!(is_safe(ex1, false), 4);
    }
}

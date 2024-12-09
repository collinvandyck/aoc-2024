#[cfg(test)]
static EX1: &str = include_str!("../../data/07/ex1");
static IN1: &str = include_str!("../../data/07/in1");

fn main() {
    println!("pt1: {}", aoc_2024::timed(|| eval(IN1, true)));
    println!("pt2: {}", aoc_2024::timed(|| eval(IN1, false)));
}

fn eval(s: &str, pt1: bool) -> usize {
    parse(s)
        .into_iter()
        .filter_map(|(test, vals)| {
            let ok = check(pt1, test, vals[0], &vals[1..]);
            ok.then_some(test)
        })
        .sum()
}

fn check(pt1: bool, target: usize, val: usize, nums: &[usize]) -> bool {
    if nums.is_empty() {
        return target == val;
    }
    let next = nums[0];
    let rest = &nums[1..];
    check(pt1, target, val + next, rest)
        || check(pt1, target, val * next, rest)
        || (!pt1 && check(pt1, target, squish(val, next), rest))
}

fn squish(mut val: usize, next: usize) -> usize {
    let mut next2 = next;
    while next2 > 0 {
        val *= 10;
        next2 /= 10;
    }
    val + next
}

fn parse(s: &str) -> Vec<(usize, Vec<usize>)> {
    s.trim()
        .lines()
        .map(|l| {
            let (test, rst) = l.split_once(":").unwrap();
            (
                test.parse().unwrap(),
                rst.split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(eval(EX1, true), 3749);
    }

    #[test]
    fn ex2() {
        assert_eq!(eval(EX1, false), 11387);
    }

    #[test]
    fn pt1() {
        assert_eq!(eval(IN1, true), 28730327770375);
    }

    #[test]
    fn pt2() {
        assert_eq!(eval(IN1, false), 424977609625985);
    }
}

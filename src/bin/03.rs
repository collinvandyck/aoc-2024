use regex::Regex;

fn main() {
    println!("pt1: {}", eval(include_str!("../../data/03/in1"), true));
    println!("pt2: {}", eval(include_str!("../../data/03/in1"), false));
}

enum Op {
    Do(bool),
    Add(isize),
}

fn eval(s: &str, pt1: bool) -> isize {
    let re = {
        let dos = (!pt1)
            .then_some(r#"do\(\)|don't\(\)|"#)
            .unwrap_or("");
        let mul = r#"mul\([0-9]{1,3},[0-9]{1,3}\)"#;
        Regex::new(&format!("{dos}{mul}")).unwrap()
    };
    s.trim()
        .lines()
        .flat_map(|line| {
            re.captures_iter(line).map(|cap| match &cap[0] {
                "do()" => Op::Do(true),
                "don't()" => Op::Do(false),
                cap => Op::Add(
                    cap[4..cap.len() - 1]
                        .split(",")
                        .map(|s| s.parse::<isize>().unwrap())
                        .product(),
                ),
            })
        })
        .fold((0, true), |(res, ok), op| match op {
            Op::Do(true) => (res, true),
            Op::Do(false) => (res, false),
            Op::Add(v) if ok => (res + v, ok),
            _ => (res, ok),
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1() {
        let s = include_str!("../../data/03/in1");
        assert_eq!(eval(s, true), 185797128);
    }

    #[test]
    fn pt2() {
        let s = include_str!("../../data/03/in1");
        assert_eq!(eval(s, false), 89798695);
    }

    #[test]
    fn ex01() {
        let s = include_str!("../../data/03/ex1");
        assert_eq!(eval(s, true), 161);
    }

    #[test]
    fn ex02() {
        let s = include_str!("../../data/03/ex2");
        assert_eq!(eval(s, false), 48);
    }
}

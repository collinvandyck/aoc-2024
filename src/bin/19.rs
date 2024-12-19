#![allow(unused)]

#[allow(unused)]
mod data {
    pub static EX1: &str = include_str!("../../data/19/ex1");
    pub static IN1: &str = include_str!("../../data/19/in1");
}

fn main() {
    println!("pt1: {}", aoc_2024::timed(|| eval(data::IN1, true)));
    println!("pt2: {}", aoc_2024::timed(|| eval(data::IN1, false)));
}

fn eval(s: &str, pt1: bool) -> usize {
    todo!("eval")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(eval(data::EX1, true), 10092);
    }
}

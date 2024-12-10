#![allow(unused)]

#[cfg(test)]
static EX1: &str = include_str!("../../data/10/ex1");
static IN1: &str = include_str!("../../data/10/in1");

fn main() {
    println!("pt1: {}", aoc_2024::timed(|| eval(IN1, true)));
    println!("pt2: {}", aoc_2024::timed(|| eval(IN1, false)));
}

fn eval(s: &str, pt1: bool) -> usize {
    0
}

use aoc_2024 as aoc;
use nom::{
    IResult,
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::digit1,
    sequence::{delimited, separated_pair},
};
use std::isize;

fn main() {
    let in1 = include_str!("../../data/03/in1");
    println!("pt1: {}", aoc::timed(|| eval_nom(in1, true)));
    println!("pt2: {}", aoc::timed(|| eval_nom(in1, false)));
}

#[derive(Debug)]
enum Op {
    Do(bool),
    Add(isize),
    Nop,
}

fn eval_nom(mut s: &str, pt1: bool) -> isize {
    fn tag_do(s: &str) -> IResult<&str, Op> {
        tag("do()")(s).map(|(rst, _)| (rst, Op::Do(true)))
    }
    fn tag_dont(s: &str) -> IResult<&str, Op> {
        tag("don't()")(s).map(|(rst, _)| (rst, Op::Do(false)))
    }
    fn nop(s: &str) -> IResult<&str, Op> {
        take(1_usize)(s).map(|(rst, _)| (rst, Op::Nop))
    }
    fn mul(s: &str) -> IResult<&str, Op> {
        let (s, _) = tag("mul")(s)?;
        let (s, (d1, d2)) = delimited(tag("("), separated_pair(digit1, tag(","), digit1), tag(")"))(s)?;
        Ok((s, Op::Add(d1.parse::<isize>().unwrap() * d2.parse::<isize>().unwrap())))
    }
    let (mut res, mut ok) = (0, true);
    while !s.is_empty() {
        match alt((tag_do, tag_dont, mul, nop))(s) {
            Ok((rst, op)) => {
                match op {
                    Op::Do(v) => ok = pt1 || v,
                    Op::Add(v) => res += if ok { v } else { 0 },
                    _ => {}
                }
                s = rst;
            }
            _ => break,
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1() {
        let s = include_str!("../../data/03/in1");
        assert_eq!(eval_nom(s, true), 185797128);
    }

    #[test]
    fn pt2() {
        let s = include_str!("../../data/03/in1");
        assert_eq!(eval_nom(s, false), 89798695);
    }

    #[test]
    fn ex01() {
        let s = include_str!("../../data/03/ex1");
        assert_eq!(eval_nom(s, true), 161);
    }

    #[test]
    fn ex02() {
        let s = include_str!("../../data/03/ex2");
        assert_eq!(eval_nom(s, false), 48);
    }
}

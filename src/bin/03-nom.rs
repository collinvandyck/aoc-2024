use aoc_2024 as aoc;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::i64 as ci64,
    sequence::{delimited, separated_pair},
};

fn main() {
    let in1 = include_str!("../../data/03/in1");
    println!("pt1: {}", aoc::timed(|| eval_nom(in1, true)));
    println!("pt2: {}", aoc::timed(|| eval_nom(in1, false)));
}

#[derive(Debug)]
enum Op {
    Do(bool),
    Add(i64),
    Nop,
}

fn eval_nom(mut s: &str, pt1: bool) -> i64 {
    fn parse_mul(s: &str) -> IResult<&str, Op> {
        let (s, _) = tag("mul")(s)?;
        let (s, (d1, d2)) = delimited(tag("("), separated_pair(ci64, tag(","), ci64), tag(")"))(s)?;
        Ok((s, Op::Add(d1 * d2)))
    }
    fn parse(s: &str) -> IResult<&str, Op> {
        alt((
            tag("do()").map(|_| Op::Do(true)),
            tag("don't()").map(|_| Op::Do(false)),
            parse_mul,
            take(1_usize).map(|_| Op::Nop),
        ))(s)
    }
    let (mut res, mut ok) = (0, true);
    while !s.is_empty() {
        match parse(s) {
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

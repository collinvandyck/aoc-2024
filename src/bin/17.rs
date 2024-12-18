#![allow(unused)]

use itertools::Itertools;

#[allow(unused)]
mod data {
    pub static EX1: &str = include_str!("../../data/17/ex1");
    pub static EX2: &str = include_str!("../../data/17/ex2");
    pub static IN1: &str = include_str!("../../data/17/in1");
}

fn main() {
    println!("pt1: {}", aoc_2024::timed(|| eval(data::IN1, true)));
    // println!("pt2: {}", aoc_2024::timed(|| eval(data::IN1, false)));
}

struct Computer {
    a: i64,
    b: i64,
    c: i64,
    program: Vec<Word>,
    pc: usize,
    output: Vec<i64>,
}

type Word = u8;

fn eval(s: &str, _pt1: bool) -> String {
    let mut c = parse(s);
    c.run();
    c.output()
}

fn reg_a(s: &str) -> i64 {
    let c = parse(s);
    let mut c = QComp::new(c);
    c.run();
    0
}

#[derive(Debug, Clone)]
enum Reg {
    Val(i64),
    Unknown,
    Pow2(Box<Reg>),
    Div { n: Box<Reg>, d: Box<Reg> },
    Mod(Box<Reg>, i64),
    Xor(Box<Reg>, Box<Reg>),
}

#[derive(Debug)]
struct QComp {
    a: Reg,
    b: Reg,
    c: Reg,
    program: Vec<Word>,
    program_idx: usize,
    pc: usize,
    output: Vec<i64>,
    equalss: Vec<Equals>,
}

#[derive(Debug, Clone)]
struct Equals {
    reg: Reg,
    val: Word,
}

impl QComp {
    fn new(computer: Computer) -> Self {
        Self {
            a: Reg::Unknown,
            b: Reg::Val(computer.b),
            c: Reg::Val(computer.c),
            program: computer.program,
            pc: computer.pc,
            output: computer.output,
            program_idx: 0,
            equalss: Vec::new(),
        }
    }
    fn run(&mut self) {
        while let Some((opcode, operand)) = self.load() {
            println!("{opcode} {operand}");
            match opcode {
                0 => self.a = self.div(operand),
                1 => {
                    let lhs = Box::new(self.b.clone());
                    let rhs = Box::new(Reg::Val(operand as i64));
                    self.b = Reg::Xor(lhs, rhs);
                }
                2 => self.b = Reg::Mod(Box::new(self.combo(operand)), 8),
                3 => {
                    println!("a: {:#?}", self.a);
                    todo!("jump");
                    //if self.a != 0 {
                    //self.pc = operand as usize;
                    //continue;
                    //}
                }
                4 => {
                    todo!("bxc")
                }
                5 => {
                    let reg = Reg::Mod(Box::new(self.combo(operand)), 8);
                    let eq = Equals {
                        reg,
                        val: self.program[self.program_idx],
                    };
                    println!("{eq:#?}");
                    self.program_idx += 1;
                }
                6 => self.b = self.div(operand),
                7 => self.c = self.div(operand),
                _ => unreachable!(),
            }
            self.pc += 2;
        }
    }
    fn div(&self, operand: Word) -> Reg {
        let n: Reg = self.a.clone();
        let d: Reg = self.combo(operand);
        let d = Reg::Pow2(Box::new(d));
        Reg::Div {
            n: Box::new(n),
            d: Box::new(d),
        }
    }
    fn print(&mut self, val: i64) {
        self.output.push(val);
    }
    fn output(&self) -> String {
        self.output
            .iter()
            .map(ToString::to_string)
            .join(",")
    }
    fn combo(&self, operand: Word) -> Reg {
        match operand {
            0..4 => Reg::Val(operand as i64),
            4 => self.a.clone(),
            5 => self.b.clone(),
            6 => self.c.clone(),
            _ => unreachable!(),
        }
    }
    fn load(&mut self) -> Option<(Word, Word)> {
        if self.pc >= self.program.len() {
            None
        } else {
            let opcode = self.program[self.pc];
            let operand = self.program[self.pc + 1];
            Some((opcode, operand))
        }
    }
}

impl Computer {
    fn run(&mut self) {
        while let Some((opcode, operand)) = self.load() {
            match opcode {
                0 => self.a = self.div(operand),
                1 => self.b = self.b ^ operand as i64,
                2 => self.b = self.combo(operand) & 7,
                3 => {
                    if self.a != 0 {
                        self.pc = operand as usize;
                        continue;
                    }
                }
                4 => self.b = self.b ^ self.c,
                5 => self.print(self.combo(operand) & 7),
                6 => self.b = self.div(operand),
                7 => self.c = self.div(operand),
                _ => unreachable!(),
            }
            self.pc += 2;
        }
    }
    fn div(&self, operand: Word) -> i64 {
        let n = self.a;
        let d = self.combo(operand);
        let d = (2 as f64).powi(d as i32) as i64;
        n / d
    }
    fn print(&mut self, val: i64) {
        self.output.push(val);
    }
    fn output(&self) -> String {
        self.output
            .iter()
            .map(ToString::to_string)
            .join(",")
    }
    fn combo(&self, operand: Word) -> i64 {
        match operand {
            0..4 => operand as i64,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }
    fn load(&mut self) -> Option<(Word, Word)> {
        if self.pc >= self.program.len() {
            None
        } else {
            let opcode = self.program[self.pc];
            let operand = self.program[self.pc + 1];
            Some((opcode, operand))
        }
    }
}

fn parse(s: &str) -> Computer {
    let mut lines = s.trim().lines();
    let reg = |s: Option<&str>| {
        let s = s.unwrap().split_once(": ").unwrap();
        s.1.parse::<i64>().unwrap()
    };
    let a = reg(lines.next());
    let b = reg(lines.next());
    let c = reg(lines.next());
    lines.next();
    let program = lines.next().unwrap().split_once(": ").unwrap().1;
    let program = program.split(",");
    let program = program.map(|s| s.parse::<u8>().unwrap());
    let program = program.collect();
    let pc = 0;
    let output = Vec::new();
    Computer {
        a,
        b,
        c,
        program,
        pc,
        output,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(eval(data::EX1, true), String::from("4,6,3,5,6,3,5,2,1,0"));
    }

    #[test]
    fn pt1() {
        assert_eq!(eval(data::IN1, true), String::from("2,1,3,0,5,2,3,7,1"));
    }

    #[test]
    fn ex2() {
        assert_eq!(reg_a(data::EX2), 117440);
    }
}

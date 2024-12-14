use itertools::Itertools;

#[allow(unused)]
mod data {
    pub static EX1: &str = include_str!("../../data/13/ex1");
    pub static IN1: &str = include_str!("../../data/13/in1");
}

fn main() {
    println!("pt1: {}", aoc_2024::timed(|| eval(data::IN1, true)));
    println!("pt2: {}", aoc_2024::timed(|| eval(data::IN1, false)));
}

fn eval(s: &str, pt1: bool) -> isize {
    Claw::parse(s).iter().map(|c| c.play(pt1)).sum()
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct XY {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy)]
struct Button {
    cost: isize,
    xy: XY,
}

#[derive(Debug)]
struct Claw {
    btn_a: Button,
    btn_b: Button,
    goal: XY,
}

impl Claw {
    fn play(&self, pt1: bool) -> isize {
        // A  = number of a presses
        // B  = number of b presses
        // ax = x delta per press of a
        // ay = y delta per press of a
        // bx = x delta per press of b
        // by = y delta per press of b
        //
        // axA + bxB = gx
        // ayA + byB = gy
        //
        // multiply each equation by the first term of the other so that we can
        // eliminate the A term.
        //
        // ay*ax*A + ay*bx*B = ay*gx
        // ax*ay*A + ax*by*B = ax*gy
        //
        // subtract the equations to eliminate the A term
        //
        // ay*bx*B - ax*by*B = ay*gx - ax*gy
        // (ay*bx - ax*by) B = ay*gx - ax*gy
        //
        // solve for B:
        // B = (ay*gx - ax*gy) / (ay*bx - ax*by)
        let (ax, ay) = (self.btn_a.xy.x, self.btn_a.xy.y);
        let (bx, by) = (self.btn_b.xy.x, self.btn_b.xy.y);
        let (gx, gy) = if pt1 {
            (self.goal.x, self.goal.y)
        } else {
            const ADJ: isize = 10000000000000;
            (self.goal.x + ADJ, self.goal.y + ADJ)
        };
        let (ax, ay, bx, by, gx, gy) = (ax as f64, ay as f64, bx as f64, by as f64, gx as f64, gy as f64);
        let b = (ay * gx - ax * gy) / (ay * bx - ax * by);
        let e = b - (b as isize) as f64;
        const EPS: f64 = 0.0001;
        if e >= EPS {
            // was not solved
            return 0;
        }
        // now that we have b, solve for a
        // axA + bxB = gx
        // axA = gx - bxB
        // A = (gx - bxB) / ax
        let a = (gx - bx * b) / ax;
        let e = a - (a as isize) as f64;
        if e >= EPS {
            // was not solved
            return 0;
        }
        let (a, b) = (a as isize, b as isize);
        a * self.btn_a.cost + b * self.btn_b.cost
    }
    fn parse(s: &str) -> Vec<Claw> {
        fn parse_xy(s: &str) -> XY {
            let a = s.split(" ").collect_vec();
            let (ax, ay) = (&a[2][2..&a[2].len() - 1], &a[3][2..]);
            let (ax, ay) = (ax.parse::<isize>().unwrap(), ay.parse::<isize>().unwrap());
            XY { x: ax, y: ay }
        }
        s.trim()
            .lines()
            .map(str::trim)
            .collect_vec()
            .chunks(4)
            .map(|vs| {
                let a = parse_xy(vs[0]);
                let a = Button { cost: 3, xy: a };
                let b = parse_xy(vs[1]);
                let b = Button { cost: 1, xy: b };
                let goal = {
                    let p = vs[2].split_whitespace().collect_vec();
                    let (x, y) = (&p[1][2..&p[1].len() - 1], &p[2][2..]);
                    let (x, y) = (x.parse().unwrap(), y.parse().unwrap());
                    XY { x, y }
                };
                Claw {
                    btn_a: a,
                    btn_b: b,
                    goal,
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exs() {
        assert_eq!(eval(data::EX1, true), 480);
    }

    #[test]
    fn pts() {
        assert_eq!(eval(data::IN1, true), 35255);
        assert_eq!(eval(data::IN1, false), 87582154060429);
    }
}

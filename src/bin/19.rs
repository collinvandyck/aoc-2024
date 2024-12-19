use hashbrown::HashMap;

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
    let prob = parse(s);
    if pt1 { prob.valid_designs() } else { prob.arrangements() }
}

struct Colors(Vec<u8>);

impl Colors {
    fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }
}

struct Problem {
    designs: Vec<Colors>,
    patterns: Vec<Colors>,
}

impl Problem {
    fn arrangements(&self) -> usize {
        type Cache<'a> = HashMap<&'a [u8], usize>;
        let mut cache = Cache::new();
        self.designs
            .iter()
            .map(|design| design.as_slice())
            .map(|design: &[u8]| {
                fn arrangements<'a>(this: &Problem, design: &'a [u8], cache: &mut Cache<'a>) -> usize {
                    if design.is_empty() {
                        return 0;
                    }
                    if let Some(&cached) = cache.get(&design) {
                        return cached;
                    }
                    this.patterns
                        .iter()
                        .map(|pat| pat.as_slice())
                        .filter_map(|pat| design.strip_prefix(pat))
                        .map(|rest| {
                            let res = rest
                                .is_empty()
                                .then_some(1)
                                .unwrap_or_else(|| arrangements(this, rest, cache));
                            cache.insert(rest, res);
                            res
                        })
                        .sum()
                }
                arrangements(self, design, &mut cache)
            })
            .sum()
    }
    fn valid_designs(&self) -> usize {
        self.designs
            .iter()
            .filter(|design| {
                fn is_valid(this: &Problem, design: &[u8]) -> bool {
                    if design.is_empty() {
                        return true;
                    }
                    this.patterns.iter().any(|p| {
                        design
                            .strip_prefix(p.as_slice())
                            .map(|rest| is_valid(this, rest))
                            .unwrap_or_default()
                    })
                }
                is_valid(self, design.as_slice())
            })
            .count()
    }
}

fn parse(s: &str) -> Problem {
    let (p1, p2) = s.trim().split_once("\n\n").unwrap();
    let patterns = p1
        .split(", ")
        .map(|s| Colors(s.as_bytes().to_vec()))
        .collect();
    let designs = p2
        .lines()
        .map(|s| Colors(s.as_bytes().to_vec()))
        .collect();
    Problem { patterns, designs }
}

impl std::fmt::Debug for Colors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.0).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(eval(data::EX1, true), 6);
    }

    #[test]
    fn pt1() {
        assert_eq!(eval(data::IN1, true), 265);
    }

    #[test]
    fn ex2() {
        assert_eq!(eval(data::EX1, false), 16);
    }
}

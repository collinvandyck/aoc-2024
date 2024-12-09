use std::{collections::VecDeque, iter::repeat_n, vec};

#[cfg(test)]
static EX1: &str = include_str!("../../data/09/ex1");
static IN1: &str = include_str!("../../data/09/in1");

fn main() {
    println!("pt1: {}", aoc_2024::timed(|| eval(IN1, true)));
    println!("pt2: {}", aoc_2024::timed(|| eval(IN1, false)));
}

fn eval(s: &str, pt1: bool) -> usize {
    let mut runs = Runs::parse(s);
    pt1.then(|| runs.compact())
        .unwrap_or_else(|| runs.defrag())
}

#[derive(Debug, Clone)]
struct Runs(VecDeque<Run>);

impl Runs {
    fn defrag(&mut self) -> usize {
        let mut ridx = self.0.len() - 1;
        while ridx > 0 {
            if let Some((bidx, bfile)) = self.find_back_file(ridx) {
                ridx = bidx - 1;
                if let Some((fidxs, fcount)) = self.find_forward_frees(bidx, bfile.count) {
                    self.0[fidxs[0]] = Run::File(bfile);
                    self.0[bidx] = Run::Free(Free { count: bfile.count });
                    let extra_frees = fidxs.len() - 1;
                    for _ in 0..extra_frees {
                        self.0.remove(fidxs[0] + 1);
                        ridx -= 1;
                    }
                    if fcount > bfile.count {
                        self.0.insert(
                            fidxs[0] + 1,
                            Run::Free(Free {
                                count: fcount - bfile.count,
                            }),
                        );
                        ridx += 1;
                    }
                }
            } else {
                break;
            }
        }
        self.checksum()
    }

    fn compact(&mut self) -> usize {
        let mut files = Vec::new();
        while let Some(back) = self.0.pop_back() {
            let mut file = match back {
                Run::Free(_) => {
                    continue;
                }
                Run::File(file) => file,
            };
            while let Some(front) = self.0.pop_front() {
                let mut free = match front {
                    Run::Free(free) => free,
                    Run::File(file) => {
                        files.push(file);
                        continue;
                    }
                };
                let delta = free.count.min(file.count);
                files.push(File { count: delta, ..file });
                file.count -= delta;
                free.count -= delta;
                if free.count > 0 {
                    self.0.push_front(Run::Free(free));
                    break;
                }
                if file.count == 0 {
                    break;
                }
            }
            if self.0.is_empty() {
                files.push(file);
            }
        }
        self.0 = VecDeque::from_iter(files.into_iter().map(Run::File));
        self.checksum()
    }

    fn checksum(&self) -> usize {
        self.0
            .iter()
            .flat_map(|r| {
                match r {
                    Run::Free(Free { count }) => repeat_n(0, *count),
                    Run::File(File { id, count }) => repeat_n(*id, *count),
                }
            })
            .enumerate()
            .map(|(idx, count)| idx * count)
            .sum()
    }

    fn find_forward_frees(&self, ridx: usize, fcount: usize) -> Option<(Vec<usize>, usize)> {
        let mut idxs = vec![];
        let mut sum = 0;
        for (idx, run) in self.0.iter().enumerate().take(ridx) {
            match run {
                Run::File(_) => {
                    if sum >= fcount {
                        break;
                    }
                    idxs.clear();
                    sum = 0;
                }
                Run::Free(free) => {
                    idxs.push(idx);
                    sum += free.count;
                }
            }
        }
        if sum >= fcount { Some((idxs, sum)) } else { None }
    }

    fn find_back_file(&self, max_idx: usize) -> Option<(usize, File)> {
        self.0
            .iter()
            .enumerate()
            .take(max_idx + 1)
            .rev()
            .find_map(|(idx, run)| {
                match run {
                    Run::Free(_) => None,
                    Run::File(file) => Some((idx, *file)),
                }
            })
    }

    fn parse(s: &str) -> Self {
        let runs = s
            .trim()
            .chars()
            .enumerate()
            .filter_map(|(idx, ch)| {
                let count = ch.to_digit(10).unwrap() as usize;
                if idx % 2 == 1 {
                    (count > 0).then_some(Run::Free(Free { count }))
                } else {
                    assert!(count > 0);
                    let id = idx / 2;
                    Some(Run::File(File { id, count }))
                }
            })
            .collect();
        Self(runs)
    }
}

#[derive(Debug, Clone, Copy, strum_macros::EnumIs)]
enum Run {
    Free(Free),
    File(File),
}

#[derive(Debug, Clone, Copy)]
struct Free {
    count: usize,
}
#[derive(Debug, Clone, Copy)]
struct File {
    id: usize,
    count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expt1() {
        assert_eq!(eval(EX1, true), 1928);
    }

    #[test]
    fn expt2() {
        assert_eq!(eval(EX1, false), 2858);
    }

    #[test]
    fn pt1() {
        assert_eq!(eval(IN1, true), 6401092019345);
    }

    #[test]
    fn pt2() {
        assert_eq!(eval(IN1, false), 6431472344710);
    }
}

use std::time::Instant;

pub fn timed<F, T>(f: F) -> T
where
    F: FnOnce() -> T,
{
    let start = Instant::now();
    let res = f();
    eprintln!("> ({:?})", start.elapsed());
    res
}

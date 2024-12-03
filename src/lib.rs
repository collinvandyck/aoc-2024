use std::time::Instant;

pub fn timed<F: FnOnce() -> T, T>(f: F) -> T {
    let start = Instant::now();
    let res = f();
    eprintln!("> ({:?})", start.elapsed());
    res
}

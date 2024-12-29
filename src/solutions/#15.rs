//Pascal Triangle and 2n-n combination

pub fn solve() -> f64 {
    let n = 20;

    let a = (n+1..=2*n).fold(1f64, |acc, v| acc * v as f64);
    let b = (1..=n).fold(1f64, |acc, v| acc * v as f64);

    a / b
}

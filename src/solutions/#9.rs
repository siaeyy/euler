pub fn solve() -> i64 {
    for c in 3..=1000 as i64 {
        for b in 2..(c - 1) {
            let a = 1000 - c - b;

            if a.pow(2) + b.pow(2) == c.pow(2) {
                return a * b * c
            }
        }
    }
    0
}

// Sieve Of Eratosthenes
// Got help from GPT for implementation :c

pub fn solve() -> u64 {
    let limit = 2_000_000;

    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    for num in 2..=((limit as f64).sqrt() as usize) {
        if is_prime[num] {
            for multiple in (num * num..=limit).step_by(num) {
                is_prime[multiple] = false;
            }
        }
    }

    is_prime
        .iter()
        .enumerate()
        .fold(0 as u64, 
            |acc, (i, b)| 
            if *b { acc + i as u64 } 
            else { acc }
        ) 
}


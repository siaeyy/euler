pub fn solve() -> u64 {
    let mut num: u64 = 600851475143;
    let mut prime_factors = Vec::new();

    while num % 2 == 0 {
        prime_factors.push(2);
        num /= 2;
    }

    let mut i = 3;
    while i * i <= num {
        while num % i == 0 {
            prime_factors.push(i);
            num /= i;
        }
        i += 2;
    }

    if num > 1 {
        prime_factors.push(num);
    }

    let greatest = prime_factors.iter().max();

    match greatest {
        Some(greatest) => *greatest,
        None => 0,
    }
}

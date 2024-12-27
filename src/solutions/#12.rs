use std::collections::HashMap;

pub fn solve() -> u32 {
    let mut map: HashMap<u32, Vec<u32>> = HashMap::new();

    fn prime_divisors(x: u32, map: &mut HashMap<u32, Vec<u32>>) {
        let mut n = x;
        let mut divisors = Vec::new();
        let mut divisor = 2;

        while n > 1 {
            while n % divisor == 0 {
                if let Some(v) = map.get(&n) {
                    divisors.extend_from_slice(v);
                    n = 1;
                    break;
                }

                divisors.push(divisor);
                n /= divisor;
            }
            divisor += 1;

            if divisor * divisor > n && n > 1 {
                divisors.push(n);
                break;
            }
        }

        map.insert(x, divisors);
    }

    let mut i = 1;
    loop {
        let n = (i * (i + 1)) / 2;
        let mut count_map: HashMap<u32, u32> = HashMap::new();

        prime_divisors(n, &mut map);

        for &number in map.get(&n).unwrap().iter() {
            *count_map.entry(number).or_insert(0) += 1;
        }

        let res = count_map.values().fold(1, |acc, v| acc * (v + 1));

        if res >= 500 {
            return n;
        }

        i += 1;
    }
}

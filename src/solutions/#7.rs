pub fn solve() -> u32 {
    let mut greatest_prime = 3;
    let mut primes = Vec::from([2, 3]);
    
    let mut i = 1;
    while i < 10_000 {
        let mut temp = greatest_prime;
        
        'prime_loop: loop {
            temp += 2;
            match primes.iter().find(|x| temp % *x == 0) {
                None => {
                    greatest_prime = temp;
                    primes.push(greatest_prime);
                    break 'prime_loop;
                }, 
                Some(_) => { continue 'prime_loop; }
            }
        }
        
        i += 1;
    }

    greatest_prime
}

pub fn solve() -> u32 {
    const LIMIT: u32 = 4_000_000;
    
    let mut i = 3;
    let mut sum = 0;

    let mut b1 = 1;
    let mut b2 = 1;

    loop {        
        (b1, b2) = (b2, b1);
        b2 = b1 + b2;
       
        if b2 > LIMIT {
            break;  
        }else if i % 3 == 0 {
            sum += b2;        
        }

        i += 1;
    }

    sum
}

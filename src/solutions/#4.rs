pub fn solve() -> u32 {
    let is_palindrome = |x: u32| {
        let txt = x.to_string();
       
        for c in txt.char_indices() {
            if c.1 != txt.chars().nth(txt.len() - c.0 - 1).unwrap() { 
                return false;
            };
        }
        true
    };
    
    let mut greatest = 0;
    for i in (100..=999).rev() {
        for j in (100..=999).rev() {
            let res = i * j;
            if is_palindrome(res) && res > greatest {
                greatest = res;
            }
        }
    }
    greatest
}

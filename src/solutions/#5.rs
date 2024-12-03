use num::Integer;

pub fn solve() -> u32 {
    let mut lcm = 1;
    
    for i in 1..=20 {
        lcm = i.lcm(&lcm);
    }

    lcm
}

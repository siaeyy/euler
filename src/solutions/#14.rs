use std::collections::HashMap;

pub fn solve() -> (u64, u64) {
    let mut map: HashMap<u64, u64> = HashMap::from([(1, 1)]);
    let mut max = (1, 1);
    
    for i in 1..1_000_000 {
        if map.contains_key(&i) {
            continue;
        }

        let mut curr = i;
        let mut chain = 1;
        loop {
            if let Some(v) = map.get(&curr) {
                chain += v;
                map.insert(i, chain);

                if chain >= max.1 {
                    max = (i, chain);
                }
                break;
            }

            if curr % 2 == 0 {
                curr = curr / 2;
            } else {
                curr = 3 * curr + 1;
            }

            chain += 1;
        }

    }
    max
}


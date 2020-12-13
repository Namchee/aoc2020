use std::io;
use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();

    let mut first: u64 = 0;
    let mut map: HashMap<u64, u32> = HashMap::new();
    let mut q: VecDeque<u64> = VecDeque::new();

    while stdin.read_line(&mut buffer).unwrap() > 0 {
        if buffer.ends_with('\n') {
            buffer.pop();
        }

        let val: u64 = buffer.parse().unwrap();

        if q.len() == 25 {
            let mut pos: bool = false;

            for (key, value) in map.iter() {
                if *key > val {
                    continue;
                }

                let pair = val - key;

                if (pair == *key && *value >= 2) || map.contains_key(&pair) {
                    pos = true;
                }
            }

            if !pos {
                first = val;
            }

            let rem = q.pop_front().unwrap();
            let freq = map.get(&rem).unwrap();

            if *freq > 1 {
                map.insert(rem, *freq - 1);
            } else {
                map.remove(&rem);
            }
        }

        if map.contains_key(&val) {
            map.insert(val, map.get(&val).unwrap() + 1);
        } else {
            map.insert(val, 1);
        }

        q.push_back(val);

        buffer.clear();
    }

    println!("{}", first);
}
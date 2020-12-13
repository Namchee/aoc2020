use std::io;
use std::vec::Vec;
use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();

    let mut first: u64 = 0;
    let mut map: HashMap<u64, u32> = HashMap::new();
    let mut q: VecDeque<u64> = VecDeque::new();

    let mut inputs: Vec<u64> = Vec::new();

    while stdin.read_line(&mut buffer).unwrap() > 0 {
        if buffer.ends_with('\n') {
            buffer.pop();
        }

        let val: u64 = buffer.parse().unwrap();
        inputs.push(val);

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

    let mut start: usize = 0;
    let mut end: usize = 0;
    let mut cur: u64 = inputs[0];

    while cur != first && start <= end && end < inputs.len() {
        if cur < first || (end - start <= 1) {
            end += 1;
            cur += inputs[end];
        } else {
            cur -= inputs[start];
            start += 1;
        }
    }

    let mut lowest = u64::MAX;
    let mut highest = 0;

    for idx in start..(end + 1) {
        let num = inputs[idx];

        if num < lowest {
            lowest = num;
        }

        if num > highest {
            highest = num;
        }
    }

    println!("{}", lowest + highest);
}
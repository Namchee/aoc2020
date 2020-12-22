use std::io;
use std::vec::Vec;
use std::collections::HashMap;

use regex::Regex;

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();

    let mut mask = String::new();
    let mut map: HashMap<u64, u64> = HashMap::new();

    while stdin.read_line(&mut buffer).unwrap() > 0 {
        if buffer.ends_with('\n') {
            buffer.pop();
        }

        let tokens: Vec<&str> = buffer.split_whitespace().collect();

        match tokens[0] {
            "mask" => {
                mask = tokens[2].to_owned();
            },
            _ => {
                let idx_raw = Regex::new(r"^mem\[(\d+)\]$").unwrap()
                    .captures(tokens[0])
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str();

                let idx: u64 = idx_raw.parse().unwrap();
                let len = mask.len();
                let mut value: u64 = tokens[2].parse().unwrap();

                for (i, val) in mask.chars().enumerate() {
                    if val == 'X' {
                        continue;
                    }

                    let masque: u64 = val as u64 - 48;

                    value &= !(1 << (len - i - 1));
                    value |= masque << (len - i - 1);
                }

                map.insert(idx, value);
            },
        }

        buffer.clear();
    }

    let mut res = 0;

    for val in map.values() {
        res += val;
    }

    println!("{}", res);
}
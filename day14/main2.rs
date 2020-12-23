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

                let idx_val: u64 = idx_raw.parse().unwrap();
                let mut idx: Vec<char> = format!("{:b}", idx_val).chars().collect();

                while idx.len() != mask.len() {
                    idx.insert(0, '0');
                }

                for (i, bit) in mask.chars().enumerate() {
                    match bit {
                        'X' => idx[i] = 'X',
                        '1' => idx[i] = '1',
                        _ => (),
                    }
                }

                let val: u64 = tokens[2].parse().unwrap();
                let indices = search_idx(idx, 0);

                for indi in indices {
                    map.insert(indi, val);
                }
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

fn search_idx(
    string: Vec<char>,
    idx: usize
) -> Vec<u64> {
    if idx == string.len() {
        let addr: String = string.iter().collect();
        let addr_val: u64 = u64::from_str_radix(&addr, 2).unwrap();

        return vec![addr_val];
    } else {
        let mut res = vec![];

        if string[idx] == 'X' {
            let mut zero = string.clone();
            let mut one = string.clone();

            zero[idx] = '0';
            one[idx] = '1';

            res.append(&mut search_idx(zero, idx + 1));
            res.append(&mut search_idx(one, idx + 1));
        } else {
            res.append(&mut search_idx(string, idx + 1));
        }

        return res;
    }
}
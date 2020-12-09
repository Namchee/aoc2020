use std::io;
use std::collections::HashMap;
use std::vec::Vec;

use regex::Regex;

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    let req = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut map: HashMap<String, String> = HashMap::new();
    let mut res: u32 = 0;

    while stdin.read_line(&mut buffer).unwrap() > 0 {
        if buffer == "\n" {
            for r in req.iter() {
                if !map.contains_key(&r.to_string()) {
                    map.insert(r.to_string(), "".to_string());
                }
            }

            if check_passport(&map) {
                res += 1;
            }

            map.clear();
        } else {
            if buffer.ends_with('\n') {
                buffer.pop();
            }

            let inputs: Vec<&str> = buffer.split_whitespace().collect();

            for key_val in inputs {
                let kval: Vec<&str> = key_val.split(':').collect();
                
                map.insert(kval[0].to_owned(), kval[1].to_owned());
            }
        }

        buffer.clear();
    }

    for r in req.iter() {
        if !map.contains_key(&r.to_string()) {
            map.insert(r.to_string(), "".to_string());
        }
    }

    if check_passport(&map) {
        res += 1;
    }

    println!("{}", res);
}

fn check_passport(map: &HashMap<String, String>) -> bool {
    let req = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let fun = vec![
        check_byr(map.get(req[0]).unwrap().to_string()),
        check_iyr(map.get(req[1]).unwrap().to_string()),
        check_eyr(map.get(req[2]).unwrap().to_string()),
        check_hgt(map.get(req[3]).unwrap().to_string()),
        check_hcl(map.get(req[4]).unwrap().to_string()),
        check_ecl(map.get(req[5]).unwrap().to_string()),
        check_pid(map.get(req[6]).unwrap().to_string())
    ];

    let mut res: bool = true;

    for verdict in fun.iter() {
        res &= verdict;
    }

    return res;
}

fn check_byr(input: String) -> bool {
    if input.is_empty() {
        return false;
    }

    let parse: i32 = input.parse().unwrap();

    return parse >= 1920 &&
        parse <= 2002;
}

fn check_iyr(input: String) -> bool {
    if input.is_empty() {
        return false;
    }

    let parse: i32 = input.parse().unwrap();

    return parse >= 2010 &&
        parse <= 2020;
}

fn check_eyr(input: String) -> bool {
    if input.is_empty() {
        return false;
    }

    let parse: i32 = input.parse().unwrap();

    return parse >= 2020 &&
        parse <= 2030;
}

fn check_hgt(input: String) -> bool {
    if input.ends_with("cm") {
        let parse: i32 = input[..input.len() - 2].parse().unwrap();

        return parse >= 150 &&
            parse <= 193;
    } else if input.ends_with("in") {
        let parse: i32 = input[..input.len() - 2].parse().unwrap();

        return parse >= 59 &&
            parse <= 76;
    } else {
        return false;
    }
}

fn check_hcl(input: String) -> bool {
    let pattern = Regex::new(r"^#[0-9a-f]{6}$").unwrap();

    return pattern.is_match(&input);
}

fn check_ecl(input: String) -> bool {
    let pattern = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();

    return pattern.is_match(&input);
}

fn check_pid(input: String) -> bool {
    let pattern = Regex::new(r"^\d{9}$").unwrap();

    return pattern.is_match(&input);
}

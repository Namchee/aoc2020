use std::io;
use std::vec::Vec;
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    let mut set = HashSet::new();
    let req = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];
    let mut res: u32 = 0;

    while stdin.read_line(&mut buffer).unwrap() > 0 {
        if buffer == "\n" {
            let mut arr: Vec<&str> = Vec::new();
    
            for r in req.iter() {    
                if !set.contains(*r) {
                    arr.push(*r);
                }
            }

            if (arr.is_empty()) || (arr.len() == 1 && arr[0] == "cid") {
                res += 1;
            }

            set.clear();
        } else {
            if buffer.ends_with('\n') {
                buffer.pop();
            }

            let inputs: Vec<&str> = buffer.split_whitespace().collect();

            for key_val in inputs {
                let kval: Vec<&str> = key_val.split(':').collect();
                
                set.insert(kval[0].to_owned());
            }
        }   
        
        buffer.clear();
    }

    let mut arr: Vec<&str> = Vec::new();
    
    for r in req.iter() {    
        if !set.contains(*r) {
            arr.push(*r);
        }
    }

    if (arr.is_empty()) || (arr.len() == 1 && arr[0] == "cid") {
        res += 1;
    }

    println!("{}", res);
}

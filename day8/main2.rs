use std::io;
use std::vec::Vec;

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();

    let mut instructions: Vec<(String, i64)> = Vec::new();
    let mut res: i64 = 0;

    while stdin.read_line(&mut buffer).unwrap() > 0 {
        if buffer.ends_with('\n') {
            buffer.pop();
        }

        let splitted: Vec<&str> = buffer.split_whitespace().collect();
        
        instructions.push((splitted[0].to_owned(), splitted[1].parse().unwrap()));

        buffer.clear();
    }

    let mut idx: usize = 0;

    while idx < instructions.len() {
        used[idx] = true;

        if instructions[idx].0 == "acc" {
            res += instructions[idx].1;
            idx += 1;
        } else if instructions[idx].0 == "jmp" {
            let next_idx = (idx as i64 + instructions[idx].1) as usize;
        } else {
            if ()
        }
    }

    println!("{}", res);
}
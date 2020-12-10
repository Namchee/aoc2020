use std::io;
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    let mut set = HashSet::new();

    let mut res: i32 = 0;

    while stdin.read_line(&mut buffer).unwrap() > 0 {
        if buffer == "\n" {
            res += set.len() as i32;

            set.clear();
        } else {
            if buffer.ends_with('\n') {
                buffer.pop();
            }

            for c in buffer.chars() {
                set.insert(c);
            }
        }

        buffer.clear();
    }

    res += set.len() as i32;

    println!("{}", res);
}
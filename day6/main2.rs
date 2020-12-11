use std::io;

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();

    let mut res: u32 = 0;

    let mut peop = 0;
    let mut char_arr: [u16; 26] = [0; 26];

    while stdin.read_line(&mut buffer).unwrap() > 0 {
        if buffer == "\n" {
            for i in 0..26 {
                if char_arr[i] == peop {
                    res += 1;
                }

                char_arr[i] = 0;
            }

            peop = 0;
        } else {
            if buffer.ends_with('\n') {
                buffer.pop();
            }

            for c in buffer.chars() {
                let ascii = (c as usize) - 97;
                char_arr[ascii] += 1;
            }

            peop += 1;
        }

        buffer.clear();
    }

    for i in 0..26 {
        if char_arr[i] == peop {
            res += 1;
        }
    }

    println!("{}", res);
}
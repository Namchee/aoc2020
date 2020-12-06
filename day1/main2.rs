use std::io;
use std::vec::Vec;

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();

    let mut res: i64 = -1;
    let mut arr = Vec::new();

    while stdin.read_line(&mut buffer).unwrap() > 0 {
        if buffer.ends_with("\n") {
            buffer.pop();
        }

        let num: i32 = buffer.parse().unwrap();

        arr.push(num);

        buffer.clear();
    }

    arr.sort();

    let len = arr.len();
    let mut it = 0;

    while (it < len - 2) && (res == -1) {
        let mut next = it + 1;
        let mut last = len - 1;

        while last > next {
            let sum = arr[it] + arr[next] + arr[last];

            if sum == 2020 {
                res = (arr[it] * arr[next] * arr[last]).into();
                break;
            } else {
                if sum > 2020 {
                    last = last - 1;
                } else {
                    next = next + 1;
                }
            }
        }

        it = it + 1;
    }

    println!("{}", res)
}
use std::io;

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();

    let mut best = 9999999;
    let mut id = 9999999;

    stdin.read_line(&mut buffer);
    buffer.pop();

    let start: i32 = buffer.parse().unwrap();

    buffer.clear();

    stdin.read_line(&mut buffer);

    let buses: Vec<&str> = buffer.split(',').collect();

    for bus in buses.iter() {
        if *bus == "x" {
            continue;
        }

        let val: i32 = bus.parse().unwrap();
        let mut mult = start / val;

        if start % val != 0 {
            mult += 1;
        }

        if mult * val < best {
            best = mult * val;
            id = val;
        }
    }

    println!("{}", (best - start) * id);
}
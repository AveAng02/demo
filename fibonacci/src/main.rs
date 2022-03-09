// Generate the nth Fibonacci number.

use std::io;

fn main() {
    let mut inp = String::new();

    io::stdin()
            .read_line(&mut inp)
            .expect("Failed to read line");

    let inp: u64 = match inp.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
    };

    println!("{}", fibo(inp, 0, 1));
}

fn fibo(mut constant_u64 : u64, num_max_u64 : u64, num_min_u64: u64) -> u64 {
    if constant_u64 > 1 {
        constant_u64 -= 1;
        fibo(constant_u64, num_max_u64 + num_min_u64, num_max_u64)
    } else {
        num_max_u64 + num_min_u64
    }
}

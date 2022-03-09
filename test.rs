//use std::io;
use std::f64;

fn main() {
    let mut x_y: f64 = 0.0;
    let mut x_x: f64 = 0.0;

    let y = [686700, 981000, 1177200, 1275300, 1471500];
    let x = [30.5, 34.3, 37.2, 37.9, 39.7];

    for element in x {
        x_x += element;
    }

    for i in 0..5 {
        x_y += (y[i] as f64).sqrt() * x[i];
    }

    let m = x_y / x_x;

    for element in x {
        println!("{}", element)
    }

    for element in x {
        println!("{}", m * element)
    }

    /*
    io::stdin()
            .read_line(&mut m)
            .expect("Failed to read line");

    let m: f64 = match m.trim().parse() {
            Ok(num) => num,
            Err(_) => 0.0,
    };

    io::stdin()
            .read_line(&mut l)
            .expect("Failed to read line");

    let l: f64 = match l.trim().parse() {
            Ok(num) => num,
            Err(_) => 0.0,
    };
*/

    println!("slope(m): {}", m);
}
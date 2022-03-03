
// Convert temperatures between Fahrenheit and Celsius.

use std::io;

fn main() {
    ///////// variables /////////
    let mut kind = String::new();
    let mut temp = String::new();
    /////////////////////////////

    println!("Enter:\nF for Celsius to Fahrenheit\nC for Fahrenheit to Celsius");

    io::stdin()
            .read_line(&mut kind)
            .expect("Failed to read line");

    let kind: char = match kind.trim().parse() {
            Ok(num) => num,
            Err(_) => ' ',
    };

    io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

    let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => 0.0,
    };

    if kind == 'F' {
        println!("\n{}F", fahrenheit(temp));
    } else if kind == 'C' {
        println!("\n{}C", celsius(temp));
    } else {
        println!("Invalid Input!!")
    } 
}

fn fahrenheit(c_temp : f64) -> f64 {
    (1.8 * c_temp) + 32.0
}

fn celsius(f_temp : f64) -> f64 {
    (5.0 * (f_temp - 32.0)) / 9.0
}

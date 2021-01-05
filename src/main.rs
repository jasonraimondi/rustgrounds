use crate::roman_numerals::convert_to_roman;
use std::io;

mod roman_numerals;

fn main() {
    let mut input = String::new();
    println!("What is your favorite number?:");
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    println!("Your favorite number is {}", convert_to_roman(n as i32));
}
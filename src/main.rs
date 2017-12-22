use std::io;
use std::io::prelude::*;

//mathmatically finds the nth fib number, bugs out after 92nd number (should probably move to i128 after it's not experimental)
fn fib(n: i64) -> i64 {
    return (((0.5) * (1.0 + (5.0 as f64).sqrt())).powf(n as f64)/(5.0 as f64).sqrt()).max(std::i64::MIN as f64).min(std::i64::MAX as f64).round() as i64;
}

fn main() {
    println!("Enter with fib number you would like:");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let s: String = line.expect("Should have more input");
        if should_quit(&s) {
            println!("Cya next time!");
            return;
        }
        match s.parse::<i64>(){
            Ok(n) => println!("fib({}) = {}",n, fib(n)),
            Err(_) => println!("Please only enter numbers or \"quit\",\"q\" or \"exit\":"),
        }
    }
}


fn should_quit(s: &String) -> bool {
    return s == "quit" || s == "q" || s == "exit";
}

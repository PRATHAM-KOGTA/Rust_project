use std::io;
use fibonacci_lib::*;
fn main() {
    // Prompt the user to enter the value of n
    println!("Enter the value of n:");

    // Read user input from the command line
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input.");

    // Parse the input to get n
    let n: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Please provide a valid integer for n.");
            return;
        }
    };
        for num  in 0..n{
       println!("Fibo({})={}",num,fib(num));
    }

}
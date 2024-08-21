// Simple Calculator
// 1. Create a command-line calculator that takes two numbers and an operator (+, -, *, /) as input.
// 2. Use functions for each operation and conditions to determine the operation based on user input.
// 3. Add error handling for division by zero.
use std::io;

pub fn calculator(){
    let mut first_number = String::new();
    let mut second_number = String::new();
    let mut operator = String::new();

    // Prompt read and covert to number first_number
    println!("Please Enter a Number : ");
    io::stdin()
        .read_line(&mut first_number)
        .expect("Failed to Read first line");
    let first_number: f64 = first_number.trim().parse().expect("Please enter a Number");

    // Prompt read and covert to number second_number
    println!("Please Enter a Second Number : ");
    io::stdin()
        .read_line(&mut second_number)
        .expect("Failed to Read first line");
    let second_number: f64 = second_number.trim().parse().expect("Please enter a Number");

    println!("Enter an operator (+, -, *, /):");
    io::stdin()
        .read_line(&mut operator)
        .expect("Failed to Read first line");

    let result: f64 = match operator.trim(){
        "+" => first_number + second_number,
        "-" => first_number - second_number,
        "*" => first_number * second_number,
        "/" => {
            if second_number != 0.0 {
                first_number / second_number
            } else{
                println!("Please enter second number non zero!!");
                return;
            }
        },
        _ => {
            println!("Invalid operator! Please use +, -, *, or /.");
            return;
        }

    };
    println!("Value of {first_number} {operator} {second_number} = {}",result);
}

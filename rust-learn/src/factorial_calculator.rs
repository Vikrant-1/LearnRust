use std::io;
pub fn calculate_factorial() {
    //  init variable

    let mut number = String::new();

    // taking input
    println!("Enter a number to calculate its factorial:");
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read number");

    // parse input
    let number: u32 = number.trim().parse::<u32>().expect("Please enter a valid number");
    // find fac
    if number <= 1 {
        println!("Factorial of {number} is 1");
        return;
    };
    let mut result: u32 = 1;
    let mut current: u32 = number;
    while current > 1 {
        result *= current;
        current -= 1;
    }

    println!("Factorial of {number} is {result}");
}

use std::io;

// Define the Operation enum
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

// Write the calculate function signature
fn calculate(operation: Operation) -> f64 {
    // Implement the calculate function using pattern matching
    match operation {
        Operation::Add(x, y) => x + y,
        Operation::Subtract(x, y) => x - y,
        Operation::Multiply(x, y) => x * y,
        Operation::Divide(x, y) => {
            if y != 0.0 {
                x / y
            } else {
                // Handling division by zero
                println!("Error: Division by zero.");
                f64::NAN
            }
        }
    }
}

fn main() {
    // Prompt the user to input numbers and operation
    println!("Enter the first number:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let first_number: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number.");
            return;
        }
    };

    println!("Enter the operation (+, -, *, /):");
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let operation_str = input.trim().to_string();

    println!("Enter the second number:");
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let second_number: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number.");
            return;
        }
    };

    // Parse user input into appropriate variables
    let operation = match operation_str.as_str() {
        "+" => Operation::Add(first_number, second_number),
        "-" => Operation::Subtract(first_number, second_number),
        "*" => Operation::Multiply(first_number, second_number),
        "/" => Operation::Divide(first_number, second_number),
        _ => {
            println!("Invalid operation.");
            return;
        }
    };

    // Call the calculate function with the created Operation enum instance
    let result = calculate(operation);

    // Print the result to the console
    println!("Result: {}", result);
}

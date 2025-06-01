use std::io::{self, Write};

#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exit,
}

impl Operation {
    fn from_choice(choice: u32) -> Option<Self> {
        match choice {
            1 => Some(Operation::Add),
            2 => Some(Operation::Subtract),
            3 => Some(Operation::Multiply),
            4 => Some(Operation::Divide),
            5 => Some(Operation::Exit),
            _ => None,
        }
    }
}

fn main() {
    fizzbuzz_challenge();
    calculator();
}

fn fizzbuzz_challenge() {
    println!("=== FizzBuzz Challenge ===");
    
    for i in 1..=20 {
        let output = match (i % 3, i % 5) {
            (0, 0) => "FizzBuzz".to_string(),
            (0, _) => "Fizz".to_string(),
            (_, 0) => "Buzz".to_string(),
            _ => i.to_string(),
        };
        println!("{}", output);
    }
}

fn calculator() {
    println!("\n=== Calculator ===");
    
    loop {
        match get_operation() {
            Some(Operation::Exit) => {
                println!("Thank you for using the calculator! Goodbye!");
                break;
            }
            Some(operation) => {
                if let Some((num1, num2)) = get_two_numbers() {
                    perform_calculation(operation, num1, num2);
                    
                    if !ask_continue() {
                        println!("Thank you for using the calculator! Goodbye!");
                        break;
                    }
                }
            }
            None => {
                println!("Invalid option. Please try again.");
            }
        }
    }
}

fn get_operation() -> Option<Operation> {
    println!("\nChoose an operation:");
    println!("1. Add");
    println!("2. Subtract");
    println!("3. Multiply");
    println!("4. Divide");
    println!("5. Exit");
    
    let choice = get_input("Enter your choice (1-5): ")?;
    let choice_num: u32 = choice.parse().ok()?;
    Operation::from_choice(choice_num)
}

fn get_two_numbers() -> Option<(f64, f64)> {
    let first = get_number("Enter the first number: ")?;
    let second = get_number("Enter the second number: ")?;
    Some((first, second))
}

fn get_number(prompt: &str) -> Option<f64> {
    loop {
        let input = get_input(prompt)?;
        match input.parse::<f64>() {
            Ok(num) => return Some(num),
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}

fn get_input(prompt: &str) -> Option<String> {
    print!("{}", prompt);
    io::stdout().flush().ok()?; // Ensure prompt is displayed before input
    
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => Some(input.trim().to_string()),
        Err(e) => {
            println!("Error reading input: {}", e);
            None
        }
    }
}

fn perform_calculation(operation: Operation, num1: f64, num2: f64) {
    let (result, symbol) = match operation {
        Operation::Add => (num1 + num2, "+"),
        Operation::Subtract => (num1 - num2, "-"),
        Operation::Multiply => (num1 * num2, "*"),
        Operation::Divide => {
            if num2 == 0.0 {
                println!("Error: Division by zero is not allowed.");
                return;
            }
            (num1 / num2, "/")
        }
        Operation::Exit => unreachable!(), // This case is handled elsewhere
    };
    
    println!("Result: {} {} {} = {}", num1, symbol, num2, result);
}

fn ask_continue() -> bool {
    loop {
        let input = match get_input("\nDo you want to perform another calculation? (yes/no): ") {
            Some(input) => input.to_lowercase(),
            None => return false,
        };
        
        match input.as_str() {
            "yes" | "y" => return true,
            "no" | "n" => return false,
            _ => println!("Please enter 'yes' or 'no'."),
        }
    }
}
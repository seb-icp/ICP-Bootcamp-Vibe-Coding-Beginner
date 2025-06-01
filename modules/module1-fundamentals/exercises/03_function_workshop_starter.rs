// TODO: 1. Define a function that adds two integers and returns the result
fn add(a: i32, b: i32) -> i32 {
    return a + b; 
}

// TODO: 2. Define a function that calculates the area of a rectangle
fn calculate_rectangle_area(width: f64, height: f64) -> f64 {
    return width * height;
}

// TODO: 3. Define a function that checks if a number is prime
fn is_prime(number: u32) -> bool {
    if number < 2 {
        return false; // Numbers less than 2 are not prime
    }
    for i in 2..=((number as f64).sqrt() as u32) {
        if number % i == 0 {
            return false; // Found a divisor, so it's not prime
        }
    }
    return true; // No divisors found, so it is prime
}

// TODO: 4. Define a function that converts Fahrenheit to Celsius
// Formula: C = (F - 32) * 5/9
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    return (fahrenheit - 32.0) * 5.0 / 9.0;
}

fn main() {
    // TODO: Call the addition function with different values and print the results
    let sum1 = add(3, 5);
    let sum2 = add(10, 20);
    
    // TODO: Calculate and print the area of rectangles with different dimensions
    let area1 = calculate_rectangle_area(5.0, 10.0);
    let area2 = calculate_rectangle_area(7.5, 3.2);
    
    // TODO: Test your prime number checker with several numbers
    let prime_check1 = is_prime(7);
    let prime_check2 = is_prime(10);
    
    // TODO: Convert and print some temperatures from Fahrenheit to Celsius
    let celsius1 = fahrenheit_to_celsius(32.0); // Freezing point of water
    let celsius2 = fahrenheit_to_celsius(212.0); // Boiling point of water
    
    // TODO: Print all results with appropriate labels
    println!("Sum of 3 and 5: {}", sum1);
    println!("Sum of 10 and 20: {}", sum2);
    println!("Area of rectangle (5.0 x 10.0): {}", area1);
    println!("Area of rectangle (7.5 x 3.2): {}", area2);
    println!("Is 7 a prime number? {}", prime_check1);
    println!("Is 10 a prime number? {}", prime_check2);
    println!("32°F in Celsius: {}", celsius1);
    println!("212°F in Celsius: {}", celsius2);
}
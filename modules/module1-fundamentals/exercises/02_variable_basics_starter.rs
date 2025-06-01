fn main() {
    // TODO: 1. Declare an immutable integer variable
    let my_integer = 42;

    // TODO: 2. Declare a mutable float variable and modify it later
    let mut my_float = 3.14; // Using f32 for float type
    
    // TODO: Modify the float value
    my_float = 2.71; // Changing the value of the mutable float variable
    
    // TODO: 3. Declare a boolean variable using type inference
    let is_rust_fun= true; // Rust infers the type as bool
    
    // TODO: 4. Declare a character variable with explicit type annotation
    let my_char: char = 'R'; // Explicitly annotating the type as char
    
    // TODO: 5. Perform arithmetic operations with the numeric variables
    let sum = 5 + 3;
    let product = 5 * 3;
    
    // TODO: 6. Print all variables and calculation results with appropriate labels
    // println!("Integer value: {}", ...);
    println!("Integer value: {}", my_integer);
    // println!("Original float value: {}", ...);
    println!("Original float value: {}", 3.14);
    // println!("Modified float value: {}", ...);
    println!("Modified float value: {}", my_float);
    // println!("Boolean value: {}", ...);
    println!("Boolean value: {}", is_rust_fun);
    // println!("Character value: {}", ...);
    println!("Character value: {}", my_char);
    // println!("Addition result: {}", ...);
    println!("Addition result: {}", sum);
    // println!("Multiplication result: {}", ...);
    println!("Multiplication result: {}", product);
}
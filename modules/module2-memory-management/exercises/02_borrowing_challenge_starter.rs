// 1. Processing string data with immutable references
fn get_length(s : &String) -> usize {
    return s.len();
}

// 2. Modifying vector data with mutable references
fn add_three_elements(vec : &mut Vec<i32>) {
    // TODO: Add three elements (10, 20, and 30) to the vector
    vec.push(10);
    vec.push(20);
    vec.push(30);
}

// 3. Processing multiple data structures
fn calculate_stats(numbers: &Vec<f64>, strings: &Vec<String>) -> (f64, i32) {
    // Calculate the average of the numbers
    let sum: f64 = numbers.iter().sum();
    let average = if !numbers.is_empty() {
        sum / numbers.len() as f64
    } else {
        0.0
    };
    
    // Count the number of strings
    let count = strings.len() as i32;
    
    return (average, count)
}

// 4. Borrowing rules demonstration
fn fix_borrowing_issues() {
    let mut data = vec![1, 2, 3];
    
    // TODO: The following code has borrowing issues. Uncomment and fix it.
    let ref1 = &mut data;
    // let ref2 = &mut data; // Cannot borrow `data` as mutable more than once at a time 
    ref1.push(4);
    ref1.push(5); // Changed to use ref1 instead of ref2
    println!("Modified data: {:?}", data);
    
    // TODO: Fix another example of borrowing issue
    let ref3 = &data;
    // let ref4 = &mut data; // Cannot borrow `data` as mutable because it is also borrowed as immutable
    // println!("Data length: {}", ref3.len());
    let ref4 = &mut data; // This is now allowed because ref3 is not used after this point
    ref4.push(6);
}

fn main() {
    // 1. Test immutable reference function
    let test_string = String::from("Hello, Rust borrowing!");
    let length = get_length(&test_string);

    println!("String length: {}", length);
    // Verify the string is still usable after passing as reference
    println!("Original string: {}", test_string);
    
    // 2. Test mutable reference function
    let mut my_vec: Vec<i32> = Vec::new();
    println!("Before function call: {:?}", my_vec);
    add_three_elements(&mut my_vec);
    println!("After function call: {:?}", my_vec);
    
    // 3. Test multiple references
    let numbers = vec![10.0, 20.0, 30.0, 40.0, 50.0];
    let words = vec![String::from("apple"), String::from("banana"), String::from("cherry")];
    let (average, count) = calculate_stats(&numbers, &words);
    println!("Average of numbers: {:.1}, Count of strings: {}", average, count);
    
    // 4. Test the fixed borrowing issues
    fix_borrowing_issues();
}
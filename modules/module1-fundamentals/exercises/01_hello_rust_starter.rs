use std::io;

fn main() {
    
    println!("Please enter your name:");

    let mut name = String::new();
    io::stdin()
    .read_line(&mut name)   
    .expect("Failed to read line");    

    println!("Hello, {}!", name.trim());

    use chrono::prelude::*;
    let now = Local::now();
    println!("Current date: {}", now.format("%Y-%m-%d").to_string());
}
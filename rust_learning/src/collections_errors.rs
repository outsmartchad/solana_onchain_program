// Collections and Error Handling Examples

use std::collections::HashMap;

// Custom error type
#[derive(Debug)]
enum MyError {
    NotFound,
    InvalidInput,
    DatabaseError,
}

// Function that returns Result
fn find_user(id: u32) -> Result<String, MyError> {
    let mut users = HashMap::new();
    users.insert(1, "Alice".to_string());
    users.insert(2, "Bob".to_string());
    
    match users.get(&id) {
        Some(name) => Ok(name.clone()),
        None => Err(MyError::NotFound),
    }
}

// Function using Option
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

// Main function to demonstrate usage
pub fn main() {
    // Vector example
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("Vector: {:?}", vec);
    
    // HashMap example
    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    println!("HashMap: {:?}", map);
    
    // Error handling with Result
    match find_user(1) {
        Ok(name) => println!("Found user: {}", name),
        Err(e) => println!("Error: {:?}", e),
    }
    
    // Error handling with Option
    match divide(10.0, 0.0) {
        Some(result) => println!("Division result: {}", result),
        None => println!("Cannot divide by zero"),
    }
    
    // Using unwrap (not recommended in production)
    let result = divide(10.0, 2.0).unwrap();
    println!("Unwrapped result: {}", result);

    
    // Using ? operator (in a function that returns Result)
    if let Err(e) = try_find_user() {
        println!("Error in try_find_user: {:?}", e);
    }
}

// Function demonstrating the ? operator
fn try_find_user() -> Result<(), MyError> {
    let user = find_user(3)?; // Will return early if error
    println!("Found user in try_find_user: {}", user);
    Ok(())
} 
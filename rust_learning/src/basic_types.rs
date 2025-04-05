// Basic Types and Ownership Examples

// Primitive Types
fn primitive_types() {
    // Integers
    let x: i32 = 42;
    let y: u64 = 100;
    
    // Floating point
    let pi: f64 = 3.14159;
    
    // Boolean
    let is_true: bool = true;
    
    // Character
    let c: char = 'A';
    
    println!("x: {}, y: {}, pi: {}, is_true: {}, c: {}", x, y, pi, is_true, c);
}

// Ownership Examples
fn ownership_examples() {
    // String (heap allocated)
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2
    // println!("s1: {}", s1); // This would cause an error!
    println!("s2: {}", s2);
    
    // Clone (deep copy)
    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("s3: {}, s4: {}", s3, s4);
    
    // References
    let s5 = String::from("reference");
    let len = calculate_length(&s5);
    let borrow_s5 = &s5;
    println!("&s5: {}", borrow_s5);
    println!("Length of '{}' is {}", s5, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn take_ownership(s: String) { // s takes the ownership
    println!("Inside function: {}", s);
} // s is dropped

fn borrow_string(s: &String) -> usize { // s is borrowed (it just borrowed a pointer from s)
    s.len() // can read but not modify 
} // s is not dropped, ownership returns to caller

fn modify_string(s: &mut String){ // mutable borrow 
    s.push_str(" So"); // it modifies the s as well
}

// Main function to run examples
pub fn main() {
    let mut s = String::from("Vincent");
    let s1 = s.clone(); // create a independent copy of s
    modify_string(&mut s);    // Mutable borrow
    println!("s1: {}", s1); 
    println!("s after modify_string(): {}", s);

    // println!("=== Primitive Types ===");
    // primitive_types();
    
    // println!("\n=== Ownership Examples ===");
    // ownership_examples();
} 
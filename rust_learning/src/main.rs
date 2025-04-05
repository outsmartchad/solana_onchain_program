// Main entry point for our Rust learning examples

mod basic_types;
mod structs_enums;
mod collections_errors;

fn main() {
    println!("=== Running Basic Types Examples ===");
    basic_types::main();
    
    println!("\n=== Running Structs and Enums Examples ===");
    structs_enums::main();
    
    println!("\n=== Running Collections and Errors Examples ===");
    collections_errors::main();
} 
// Structs and Enums Examples

// Define a struct
struct Person {
    name: String,
    age: u32,
    is_student: bool,
}

// Define an enum
enum Status {
    Active,
    Inactive,
    Suspended,
}

// Enum with data
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Implementation for Person
impl Person {
    // Constructor
    fn new(name: String, age: u32, is_student: bool) -> Self {
        Self {
            name,
            age,
            is_student,
        }
    }
    
    // Method
    fn greet(&self) {
        println!("Hello, my name is {} and I am {} years old.", self.name, self.age);
    }
    
    // Method that takes ownership
    fn into_name(self) -> String {
        self.name
    }
}

// Main function to demonstrate usage
pub fn main() {
    // Create a person
    let person = Person::new(String::from("Vincent"), 22, true);
    person.greet();
    
    // Use enum
    let status = Status::Inactive;
    match status {
        Status::Active => println!("User is active"),
        Status::Inactive => println!("User is inactive"),
        Status::Suspended => println!("User is suspended"),
    }
    
    // Use enum with data
    //let msg = Message::Write(String::from("Hello, world!"));
    // let msg = Message::Move{x:1, y:-2};
    //let msg = Message::Quit;
    let msg = Message::ChangeColor(255, 255, 255);
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Write: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
    }
    
    // Demonstrate ownership
    let name = person.into_name();
    println!("Person's name: {}", name);
    // person.greet(); // This would cause an error because person was moved!
} 
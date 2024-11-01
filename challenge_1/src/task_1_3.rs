// Structs
#[derive(Debug)]
pub struct Student {
    pub name: String,
    pub age: u32,
    pub gpa: f32,
}

impl Student {
    // Function to create a new Student instance
    pub fn new_student(name: String, age: u32, gpa: f32) -> Student {}

    // Method to display student information
    pub fn display(&self) {}
}

// Enums
#[derive(Debug)]
pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    // Function to return the duration of each light
    pub fn light_duration(&self) -> u32 {}
}

// Option Enum
// Function to safely divide two integers
pub fn safe_divide(a: i32, b: i32) -> Option<i32> {}

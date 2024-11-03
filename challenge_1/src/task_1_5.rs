use std::fs::{File};
use std::io::{self, BufRead, Write};
use std::path::Path;

// Function to parse a string into an integer, returns Result type
pub fn parse_integer(s: &str) -> Result<i32, String> {
    s.parse::<i32>().map_err(|e|e.to_string())
}

// File I/O

// Function to read all lines from a file, returns Result type with Vec<String>
pub fn read_file_lines<P: AsRef<Path>>(file_path: P) -> Result<Vec<String>, io::Error> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    
    let lines: Result<Vec<String>, io::Error> = reader.lines().collect();
    
    lines
}

// Function to write a string to a file, returns Result type
pub fn write_file<P: AsRef<Path>>(file_path: P, content: &str) -> Result<(), io::Error> {
    let mut file = File::create(file_path)?;

    file.write_all(content.as_bytes())?;

    Ok(())
}

// Combining Error Handling and File I/O

// Function to read integers from file, compute their sum, and return the result
pub fn read_and_sum_integers<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
    let lines = read_file_lines(file_path).map_err(|e|e.to_string())?;
    
    let sum = lines.iter().map(|line|parse_integer(line))
        .sum::<Result<i32,String>>()?;

    Ok(sum)
}

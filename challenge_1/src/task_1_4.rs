use std::collections::HashMap;

// Function to square each element in a vector and return a new vector
pub fn square_elements(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|item|item.pow(2)).collect()
}

// HashMap

// Function to print population of a given city or a not found message
pub fn print_population(city_population: &HashMap<String, i32>, city: &str) {
    city_population.get(city).map_or_else(
        || println!("City not found"),
        |population| println!("Population of {}: {}", city, population),
    );
}

// Function to filter even numbers from a vector using an iterator
pub fn filter_even_numbers(v: &Vec<i32>) -> Vec<i32> {
    v.iter().filter(|&item| item % 2 == 0).cloned().collect()
}

// Function to sum odd numbers in a vector using an iterator
pub fn sum_odd_numbers(v: &Vec<i32>) -> i32 {
    v.iter().filter(|&item| item % 2 != 0).sum()
}

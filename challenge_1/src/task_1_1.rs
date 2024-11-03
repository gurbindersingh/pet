// Conditional Statements: max_of_three function
pub fn max_of_three(a: i32, b: i32, c: i32) -> i32 {
    let mut max = a;

    if b > max {
        max = b;
    }

    if c > max {
        max = c;
    }

    max
}

// Loops: sum_even_numbers function
pub fn sum_even_numbers() -> i32 {
   let mut sum = 0; 
   
    for val in 1..=100 {
        if val % 2 == 0 {
            sum += val;
        }
    }

    sum
}

// Loops: Countdown function with while loop
pub fn countdown() {
    let mut countdown = 10;

    while countdown > 0 {
        println!("{}", countdown);
        countdown -= 1;
    }
    println!("Liftoff!");
}

// Match Statement: day_of_week function
pub fn day_of_week(day: i32) -> &'static str {
    match day {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        7 => "Sunday",
        _ => "Invalid day"
    }
}

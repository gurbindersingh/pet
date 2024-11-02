// Conditional Statements: max_of_three function
pub fn max_of_three(a: i32, b: i32, c: i32) -> i32 {
    let mut max = a;

    if b > max {
        max = b;
    }

    if c > max {
        max = c;
    }

    return max;
}

// Loops: sum_even_numbers function
pub fn sum_even_numbers() -> i32 {
   let mut sum = 0; 
   
    for val in 1..=100 {
        if val % 2 == 0 {
            sum += val;
        }
    }

    return sum;
}

// Loops: Countdown function with while loop
pub fn countdown() {
    let mut countdown = 10;

    while countdown > 0 {
        println!("{}", countdown);
        countdown -= 1;
    }
    println!("Liftoff");
}

// Match Statement: day_of_week function
pub fn day_of_week(day: i32) -> &'static str {
    match day {
        1 => return "Monday",
        2 => return "Tuesday",
        3 => return "Wednesday",
        4 => return "Thursday",
        5 => return "Friday",
        6 => return "Saturday",
        7 => return "Sunday",
        _ => return "Invalid day"
    }
}

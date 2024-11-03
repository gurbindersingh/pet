use task_1_1::{countdown, day_of_week, max_of_three, sum_even_numbers};
use task_1_2::{concat_strings, factorial, find_max, is_prime, reverse_string};
use task_1_3::{safe_divide, Student, TrafficLight};

mod task_1_1;
mod task_1_2;
mod task_1_3;
// mod task_1_4;
mod task_1_5;

fn main() {
    println!("Hello World");

    println!("Testing Task 1:");
    println!("{}", max_of_three(5, -20, 100));
    assert_eq!(max_of_three(5, -20, 100), 100);

    println!("{}", factorial(5));
    assert_eq!(factorial(5), 120);

    println!("{}", sum_even_numbers());
    assert_eq!(sum_even_numbers(), 2550);

    countdown();

    println!("{}", day_of_week(7));
    assert_eq!(day_of_week(7), "Sunday");

    println!("Testing Task 2:");

    println!("{}", is_prime(11));
    assert_eq!(is_prime(11), true);

    let mut s= String::from("reverse");
    println!("{:?}",reverse_string(&mut s));
    assert_eq!(s, "esrever");

    println!("{}", concat_strings("part1", "part2"));
    assert_eq!(concat_strings("part1", "part2"), "part1part2");

    let mut arr = [3, 1, 5, 2, 6, 8, 3, 4, 6, 8, 10];
    let slice = &mut arr[..9];
    println!("{:?}", find_max(slice));
    //assert_eq!(find_max(slice), );
    Student::new_student(String::from("Foo Bar"), 20, 3.9).display();
    assert_eq!(TrafficLight::Red.light_duration(), 60);
    assert_eq!(TrafficLight::Green.light_duration(), 30);
    assert_eq!(TrafficLight::Yellow.light_duration(), 5);
    assert_eq!(safe_divide(4, 2), Some(2));
    assert_eq!(safe_divide(4, 0), None);
}

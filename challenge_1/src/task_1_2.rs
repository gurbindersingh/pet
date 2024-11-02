// Function: factorial (recursive)
pub fn factorial(n: u32) -> u32 {
    if n == 0 || n == 1{
        return 1;
    } else {
        return n*factorial(n-1);
    }
}

// Function: is_prime (using factorial function)
pub fn is_prime(n: u32) -> bool {
    if n == 1{
        return false;
    }
    return factorial(n-1) % n == n-1;
}

// Ownership and Borrowing

// Function: reverse_string (mutably borrow and reverse in place) -> correct?
pub fn reverse_string(s: &mut String) {
    let s_chars: &mut [char] = &mut s.chars().collect::<Vec<_>>();

    s_chars.reverse();

    *s = s_chars.iter().collect();
    
}

// Function: concat_strings (concatenates two &str and returns a String)
pub fn concat_strings(s1: &str, s2: &str) -> String {
    let mut result = s1.to_string();
    result.push_str(s2);
    return result;
}

// Slices

// Function: find_max (finds the maximum value in a slice of integers) -> correct?
pub fn find_max(slice: &[i32]) -> Option<i32> {
    slice.iter().cloned().max()
}

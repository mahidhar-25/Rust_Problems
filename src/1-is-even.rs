fn main() {
    println!("{}", is_even(20));
}

// Function to check if a number is even
// Returns true if the number is even, false otherwise
fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    }
    return false;
}

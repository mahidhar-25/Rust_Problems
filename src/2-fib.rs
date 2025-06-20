fn main() {
    for i in 0..10 {
        println!("Fibonacci of {} is {}", i, fibonacci(i));
    }
}
//for loop in rust

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

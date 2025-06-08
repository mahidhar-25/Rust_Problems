fn main() {
    let my_string = String::from("Hello World");
    let length = get_length_of_the_string(&my_string);
    println!("The length of '{}' is {}", my_string, length);
}

fn get_length_of_the_string(s: &str) -> usize {
    s.chars().count()
}

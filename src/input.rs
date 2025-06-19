use std::io;

fn get_input() -> io::Result<String> {
    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}

fn main() {
    let mut all_input = vec![];
    while all_input.len() < 3 {
        match get_input() {
            Ok(word) => {
                let x: i32 = word.parse().expect("input not an integer");
                all_input.push(x);
            }
            Err(e) => {
                println!("error : {:?}", e)
            }
        }
    }

    for input in all_input {
        println!("Original : {:?}", input);
    }
}

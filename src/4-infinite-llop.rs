fn main() {
    let mut a = 1;
    loop {
        if a == 5 {
            break;
        }

        println!("{:?}", a);
        a += 1;
    }
}

#[derive(Debug)]
struct Adult {
    name: String,
    age: i32,
}

impl Adult {
    fn is_age_valid_to_vote(&self) {
        // You can't use expressions in `match` arms like this
        // Use an `if` statement instead
        if self.age > 21 {
            println!("eligible to vote");
        } else {
            println!("not eligible to vote");
        }
    }

    fn new(name: String, age: i32) -> Result<Adult, String> {
        if age < 21 {
            return Err("Age must be at least 21".to_string());
        }
        if name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        Ok(Adult { name, age })
    }
}

fn main() {
    let alice = Adult::new("Alice".to_string(), 21);
    match alice {
        Ok(adult) => {
            println!("Adult created: {:?}", adult);
            adult.is_age_valid_to_vote();
        }
        Err(e) => {
            println!("Error creating adult: {}", e);
        }
    }
    let bob = Adult::new("Bob".to_string(), 20);
    match bob {
        Ok(adult) => {
            println!("Adult created: {:?}", adult);
            adult.is_age_valid_to_vote();
        }
        Err(e) => {
            println!("Error creating adult: {}", e);
        }
    }
}

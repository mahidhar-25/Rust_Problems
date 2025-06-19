#[derive(Debug, PartialEq)]
struct Adult {
    name: String,
    age: i32,
}

impl Adult {
    fn new(name: String, age: i32) -> Result<Adult, String> {
        if age < 21 {
            return Err("Age must be at least 21".to_string());
        }
        if name.trim().is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        Ok(Adult { name, age })
    }

    fn is_age_valid_to_vote(&self) -> bool {
        self.age > 21
    }
}

fn main() {
    // Optional: leave empty or use for manual testing
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_adult() {
        let adult = Adult::new("John".to_string(), 30);
        assert!(adult.is_ok());
        assert_eq!(
            adult.unwrap(),
            Adult {
                name: "John".to_string(),
                age: 30
            }
        );
    }

    #[test]
    fn test_underage_adult() {
        let adult = Adult::new("Teen".to_string(), 18);
        assert!(adult.is_err());
        assert_eq!(adult.unwrap_err(), "Age must be at least 21");
    }

    #[test]
    fn test_empty_name() {
        let adult = Adult::new("   ".to_string(), 25);
        assert!(adult.is_err());
        assert_eq!(adult.unwrap_err(), "Name cannot be empty");
    }

    #[test]
    fn test_vote_eligibility() {
        let adult = Adult::new("Alice".to_string(), 22).unwrap();
        assert!(adult.is_age_valid_to_vote());
    }

    #[test]
    fn test_vote_ineligibility() {
        let adult = Adult::new("Bob".to_string(), 21).unwrap();
        assert!(!adult.is_age_valid_to_vote());
    }
}

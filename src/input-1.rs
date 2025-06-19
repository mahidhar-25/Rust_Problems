use std::{collections::HashMap, io};

fn get_input() -> io::Result<String> {
    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_lowercase())
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum PowerStates {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

fn main() {
    // Mapping enum to messages
    let power_states_map: HashMap<PowerStates, &str> = HashMap::from([
        (PowerStates::Off, "Power is getting off"),
        (PowerStates::Sleep, "Going for sleep"),
        (PowerStates::Reboot, "System is getting Rebooted"),
        (PowerStates::Shutdown, "Shutting down"),
        (PowerStates::Hibernate, "System is going into hibernate"),
    ]);

    // Mapping string inputs to enum variants
    let str_to_state: HashMap<&str, PowerStates> = HashMap::from([
        ("off", PowerStates::Off),
        ("sleep", PowerStates::Sleep),
        ("reboot", PowerStates::Reboot),
        ("shutdown", PowerStates::Shutdown),
        ("hibernate", PowerStates::Hibernate),
    ]);

    let mut all_input = vec![];

    println!("Enter 3 power states (off, sleep, reboot, shutdown, hibernate):");

    while all_input.len() < 3 {
        match get_input() {
            Ok(word) => {
                if let Some(state) = str_to_state.get(word.as_str()) {
                    let message = power_states_map.get(state).unwrap();
                    println!("{}", message);
                    all_input.push(state);
                } else {
                    println!("No such power state exists: {}", word);
                }
            }
            Err(e) => {
                println!("Error reading input: {:?}", e);
            }
        }
    }

    println!("All inputs processed: {:?}", all_input);
}

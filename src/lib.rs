
//goal is to have a console application of buying items and calculkatinf count and display.
use std::io;

use std::collections::HashMap;
#[derive(Debug)]
struct Item {
    name: String,
    price: f64,
    quantity: u32,
}

struct ShoppingCart {
    items: HashMap<String, Item>,
}

impl ShoppingCart {
    fn new() -> Self {
        ShoppingCart {
            items: HashMap::new(),
        }
    }

    fn add_item(&mut self, name: String, price: f64, quantity: u32) {
        let item = Item {
            name,
            price,
            quantity,
        };
        // Check if the item already exists in the cart
        if let Some(existing_item) = self.items.get_mut(&item.name) {
            // If it exists, update the quantity
            existing_item.quantity += item.quantity;
        } else {
            // If it doesn't exist, insert the new item
            self.items.insert(item.name.clone(), item);
        }
    }

    fn remove_item(&mut self, name: &str) {
        self.items.remove(name);
    }

    fn update_item(&mut self, name: &str, price: f64, quantity: u32) {
        if let Some(item) = self.items.get_mut(name) {
            item.price = price;
            item.quantity = quantity;
        }
    }

    fn show_cart(&self) {
        for item in self.items.values() {
            println!("{} - ${:.2} x {}", item.name, item.price, item.quantity);
        }
    }

    fn total_price(&self) -> f64 {
        self.items
            .values()
            .map(|item| item.price * (item.quantity as f64))
            .sum()
    }
}
fn get_input() -> io::Result<String> {
    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}

fn print_menu() {
    println!("Welcome to the Shopping Cart!");
    println!("Please select an action:");
    println!("1. Add Item");
    println!("2. Remove Item");
    println!("3. Update Item");
    println!("4. Show Cart");
    println!("5. Give me total price");
    println!("7. Exit");

    let mut shopping_cart = ShoppingCart::new();
    //print menu on every loop iteration

    while let Ok(input) = get_input() {
        match input.as_str() {
            "1" => {
                println!("Enter item name:");
                let name = get_input().unwrap();
                println!("Enter item price:");
                let price: f64 = get_input().unwrap().parse().unwrap_or(0.0);
                println!("Enter item quantity:");
                let quantity: u32 = get_input().unwrap().parse().unwrap_or(1);
                shopping_cart.add_item(name, price, quantity);
            }
            "2" => {
                println!("Enter item name to remove:");
                let name = get_input().unwrap();
                shopping_cart.remove_item(&name);
            }
            "3" => {
                println!("Enter item name to update:");
                let name = get_input().unwrap();
                println!("Enter new price:");
                let price: f64 = get_input().unwrap().parse().unwrap_or(0.0);
                println!("Enter new quantity:");
                let quantity: u32 = get_input().unwrap().parse().unwrap_or(1);
                shopping_cart.update_item(&name, price, quantity);
            }
            "4" => shopping_cart.show_cart(),
            "5" => {
                let total = shopping_cart.total_price();
                println!("Total price: ${:.2}", total);
            }
            "7" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid option, please try again."),
        }
        println!("Please select an action:");
        println!("1. Add Item");
        println!("2. Remove Item");
        println!("3. Update Item");
        println!("4. Show Cart");
        println!("5. Give me total price");
        println!("7. Exit");
    }
}

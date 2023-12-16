use chrono::{DateTime, Local, Utc};
use std::collections::HashMap;
use std::io::{self, Write};
use uuid::Uuid;

struct OrderItem {
    name: String,
    quantity: u32,
    price: f64,
    timestamp: DateTime<Utc>,
}

struct Order {
    id: Uuid,
    items: Vec<OrderItem>,
    order_total: f64,
    taxes: f64,
    tip: f64,
    amount_owed: f64,
    server_id: u32,
}

impl Order {
    fn new(server_id: u32) -> Self {
        Order {
            id: Uuid::new_v4(),
            items: Vec::new(),
            order_total: 0.0,
            taxes: 0.102, // 10.2% taxes
            tip: 0.0,
            amount_owed: 0.0,
            server_id,
        }
    }

    fn add_item(&mut self, item: &str, quantity: u32, price: f64) {
        let timestamp = Utc::now();
        let order_item = OrderItem {
            name: item.to_string(),
            quantity,
            price,
            timestamp,
        };
        self.items.push(order_item);
        self.order_total += price * quantity as f64;
        self.order_total = (self.order_total * 100.0).round() / 100.0;
    }

    fn remove_item(&mut self, item_index: usize) {
        if item_index < self.items.len() {
            let removed_item = self.items.remove(item_index);
            self.order_total -=
                removed_item.price * removed_item.quantity as f64;
            self.order_total = (self.order_total * 100.0).round() / 100.0;
        }
    }

    fn add_tip(&mut self, tip: f64) {
        self.tip = tip;
    }

    fn calculate_amount_owed(&mut self) {
        self.amount_owed = self.order_total + (self.order_total * self.taxes) + self.tip;
        self.amount_owed = (self.amount_owed * 100.0).round() / 100.0;
    }

    fn display_order(&self) {
        println!("┌─────────────────────────────────────────────────────────────┐");
        println!("│ Order ID: {}", self.id);
        println!("│ Server ID: {}", self.server_id);
        println!("│ Order Date: {}", Local::now().format("%Y-%m-%d"));
        println!("├─────────────────────────────────────────────────────────────┤");
        println!("│ Order Details:                                              │");
        println!("├─────────────────────────────────────────────────────────────┤");
        println!("│ No.  │ Item       │ Quantity │ Price  │ Date                │");
        println!("├──────┼────────────┼──────────┼────────┼─────────────────────┤");
        for (index, item) in self.items.iter().enumerate() {
            println!(
                "│ {:<4} │ {:<10} │ {:<8} │ ${:<5.2} │ {} │",
                index + 1,
                item.name,
                item.quantity,
                item.price,
                item.timestamp
                    .with_timezone(&Local)
                    .format("%Y-%m-%d %H:%M:%S")
            );
        }
        println!("├──────┴────────────┴──────────┴────────┴─────────────────────┤");
        println!(
            "│ Order Total: ${:<8.2}                                      │",
            self.order_total
        );
        println!(
            "│ Taxes: ${:<8.2}                                            │",
            self.order_total * self.taxes
        );
        println!(
            "│ Tip: ${:<8.2}                                              │",
            self.tip
        );
        println!(
            "│ Amount Owed: ${:<8.2}                                      │",
            self.amount_owed
        );
        println!("└─────────────────────────────────────────────────────────────┘");
    }
}

fn main() {
    let mut order = Order::new(123); // Example server ID

    let menu: HashMap<&str, f64> = [
        ("Burger", 9.99),
        ("Pizza", 12.99),
        ("Salad", 7.99),
        ("Fries", 3.99),
        ("Spaghetti", 10.99),
        ("Lasagna", 11.99),
        ("Ravioli", 9.99),
        ("Tiramisu", 6.99),
        ("Coke", 1.99),
        ("Coffee", 2.49),
        ("Combo Meal", 15.99),
        ("Kids Meal", 8.99),
    ]
    .iter()
    .cloned()
    .collect();

    println!("Menu:");
    println!("{:<12} {:<6}", "Item", "Price");
    println!("-------------------");
    for (item, price) in &menu {
        println!("{:<12} ${:.2}", item, price);
    }
    println!();

    loop {
        println!("User Selection Options:");
        println!("1. Add item to order");
        println!("2. Remove item from order");
        println!("3. Add tip");
        println!("4. Display order");
        println!("5. Exit");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        match choice.trim().parse::<u32>() {
            Ok(1) => {
                print!("Enter item name: ");
                io::stdout().flush().unwrap();
                let mut item = String::new();
                io::stdin().read_line(&mut item).unwrap();

                let item = item.trim();
                if !menu.contains_key(item) {
                    println!("Invalid item. Please try again.");
                    continue;
                }

                print!("Enter quantity: ");
                io::stdout().flush().unwrap();
                let mut quantity = String::new();
                io::stdin().read_line(&mut quantity).unwrap();

                let quantity = match quantity.trim().parse::<u32>() {
                    Ok(qty) => qty,
                    Err(_) => {
                        println!("Invalid quantity. Please try again.");
                        continue;
                    }
                };

                let price = *menu.get(item).unwrap();
                order.add_item(item, quantity, price);
                println!("Item added to order.");
            }
            Ok(2) => {
                if order.items.is_empty() {
                    println!("Order is empty.");
                    continue;
                }

                println!("Current Order:");
                for (index, item) in order.items.iter().enumerate() {
                    println!(
                        "{}. {} x {} - ${:.2}",
                        index + 1,
                        item.name,
                        item.quantity,
                        item.price
                    );
                }

                print!("Enter the item number to remove: ");
                io::stdout().flush().unwrap();
                let mut item_number = String::new();
                io::stdin().read_line(&mut item_number).unwrap();

                let item_number = match item_number.trim().parse::<usize>() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid item number. Please try again.");
                        continue;
                    }
                };

                order.remove_item(item_number - 1);
                println!("Item removed from order.");
            }
            Ok(3) => {
                if order.order_total > 0.0 {
                    print!("Enter tip amount: $");
                    io::stdout().flush().unwrap();
                    let mut tip_amount = String::new();
                    io::stdin().read_line(&mut tip_amount).unwrap();

                    let tip_amount = match tip_amount.trim().parse::<f64>() {
                        Ok(amount) => amount,
                        Err(_) => {
                            println!("Invalid tip amount. Please try again.");
                            continue;
                        }
                    };

                    order.add_tip(tip_amount);
                    println!("Tip added to order.");
                } else {
                    println!("No items in the order. Cannot add tip.");
                }
            }
            Ok(4) => {
                if order.items.is_empty() {
                    println!("Order is empty.");
                } else {
                    order.calculate_amount_owed();
                    order.display_order();
                }
            }
            Ok(5) => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid choice. Please try again.");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_item() {
        let mut order = Order::new(123);
        order.add_item("Burger", 2, 9.99);

        assert_eq!(order.items.len(), 1);
        assert_eq!(order.order_total, 19.98);
    }

    #[test]
    fn test_remove_item() {
        let mut order = Order::new(123);
        order.add_item("Burger", 2, 9.99);
        println!("Order Total: {}", order.order_total);
        order.add_item("Pizza", 1, 12.99);
        println!("Order Total: {}", order.order_total);
        order.remove_item(0);
        println!("Order Total: {}", order.order_total);

        assert_eq!(order.items.len(), 1);
        assert_eq!(order.order_total, 12.99);
    }

    #[test]
    fn test_add_tip() {
        let mut order = Order::new(123);
        order.add_item("Burger", 2, 9.99);
        order.add_tip(2.0);

        assert_eq!(order.tip, 2.0);
    }

    #[test]
    fn test_calculate_amount_owed() {
        let mut order = Order::new(123);
        order.add_item("Burger", 2, 9.99);
        order.add_tip(2.0);
        order.calculate_amount_owed();

        assert_eq!(order.amount_owed, 24.02);
    }
}

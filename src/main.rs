#![allow(unused_imports)]
mod inventory;
mod transactions;
mod security;

use std::io::{self, Write};

use inventory::{Inventory, report_inventory};
use transactions::{Transactions, report_sales, report_purchases};
use security::{login, signup, change_password, Role, User};

fn prompt(label: &str) -> String {
    print!("{label}");
    let _ = io::stdout().flush();
    let mut s = String::new();
    let _ = io::stdin().read_line(&mut s);
    s.trim().to_string()
}

fn seed_inventory(inv: &mut Inventory) {
    // hardcoded sample items
    inv.add_product("Cola", "330ml can", 10.0, 12);
    inv.add_product("Chips", "Salted", 7.5, 5);
    inv.add_product("Bread", "Whole grain", 18.0, 10);
}

fn client_menu(inv: &mut Inventory, tx: &mut Transactions, user: &User) {
    loop {
        println!("\n== Client Menu ==");
        println!("1) View products");
        println!("2) Purchase");
        println!("3) Change password");
        println!("0) Logout");
        match prompt("> ").as_str() {
            "1" => println!("{}", report_inventory(inv)),
            "2" => {
                let name = prompt("Product name: ");
                let qty: u32 = prompt("Quantity: ").parse().unwrap_or(0);
                let unit: f64 = prompt("Unit price: ").parse().unwrap_or(0.0);
                if qty == 0 || unit <= 0.0 {
                    println!("Invalid quantity or price.");
                    continue;
                }
                match tx.record_sale(&name, qty, unit, inv) {
                    Ok(s) => println!("Sale: {} x{} = {}", s.product_name(), s.quantity(), s.total_price()),
                    Err(e) => println!("Error: {:?}", e),
                }
            }
            "3" => change_password(user),
            "0" => break,
            _ => println!("Invalid choice."),
        }
    }
}

fn manager_menu(inv: &mut Inventory, tx: &mut Transactions, user: &User, with_reports: bool) {
    loop {
        println!("\n== Manager Menu ==");
        println!("1) View products");
        println!("2) Add product");
        println!("3) Edit product");
        println!("4) Delete product");
        println!("5) Record supplier purchase");
        if with_reports {
            println!("6) View reports");
            println!("7) Change password");
        } else {
            println!("6) Change password");
        }
        println!("0) Logout");

        let choice = prompt("> ");
        match choice.as_str() {
            "1" => println!("{}", report_inventory(inv)),
            "2" => {
                let name = prompt("Name: ");
                let desc = prompt("Description: ");
                let price: f64 = prompt("Price: ").parse().unwrap_or(0.0);
                let qty: u32 = prompt("Quantity: ").parse().unwrap_or(0);
                inv.add_product(&name, &desc, price, qty);
                println!("Added.");
            }
            "3" => {
                let current = prompt("Product to edit (current name): ");
                let name_in = prompt("New name (leave empty to keep): ");
                let desc_in = prompt("New description (leave empty to keep): ");
                let price_in = prompt("New price (leave empty to keep): ");
                let qty_in = prompt("New quantity (leave empty to keep): ");
                let name_opt = if name_in.is_empty() { None } else { Some(name_in.as_str()) };
                let desc_opt = if desc_in.is_empty() { None } else { Some(desc_in.as_str()) };
                let price_opt = if price_in.is_empty() { None } else { price_in.parse().ok() };
                let qty_opt = if qty_in.is_empty() { None } else { qty_in.parse().ok() };
                inv.edit_product(&current, name_opt, desc_opt, price_opt, qty_opt);
                println!("Updated (if product existed).");
            }
            "4" => {
                let name = prompt("Product to delete: ");
                inv.del_product(&name);
                println!("Removed (if product existed).");
            }
            "5" => {
                let name = prompt("Product: ");
                let qty: u32 = prompt("Quantity: ").parse().unwrap_or(0);
                let unit: f64 = prompt("Unit price: ").parse().unwrap_or(0.0);
                match tx.record_purchase(&name, qty, unit, inv) {
                    Ok(p) => println!("Purchase: {} x{} = {}", p.product_name(), p.quantity(), p.total_cost()),
                    Err(e) => println!("Error: {:?}", e),
                }
            }
            "6" if with_reports => {
                println!("-- Inventory --\n{}", report_inventory(inv));
                println!("-- Sales --\n{}", report_sales(tx));
                println!("-- Purchases --\n{}", report_purchases(tx));
            }
            "7" if with_reports => change_password(user),
            "6" if !with_reports => change_password(user),
            "0" => break,
            _ => println!("Invalid choice."),
        }
    }
}

fn prelogin() -> Option<User> {
    loop {
        println!("\n== Welcome ==");
        println!("1) Log in");
        println!("2) Sign up (Client)");
        println!("0) Exit");
        match prompt("> ").as_str() {
            "1" => {
                if let Some(u) = login() { return Some(u); }
            }
            "2" => signup(),
            "0" => return None,
            _ => println!("Invalid choice."),
        }
    }
}

fn main() {
    // Login or sign up
    let Some(user) = prelogin() else {
        println!("Goodbye.");
        return;
    };

    // Init store state
    let mut inv = Inventory::new();
    seed_inventory(&mut inv);
    let mut tx = Transactions::new();

    // Route by role
    match user.role {
        Role::Client  => client_menu(&mut inv, &mut tx, &user),
        Role::Manager => manager_menu(&mut inv, &mut tx, &user, false),
        Role::Admin   => manager_menu(&mut inv, &mut tx, &user, true),
    }

    println!("Goodbye!");
}
#![allow(unused_imports)]
mod inventory;
mod transactions;
mod security;

use inventory::{Inventory, report_inventory};
use transactions::{Transactions, report_sales, report_purchases, StoreError};
use security::{require_login};

fn main() {
    if !require_login() {
        eprintln!("Access denied. Exiting.");
        return;
    }
    
    let mut inv = Inventory::new();
    inv.add_product("Cola", "330ml can", 10.0, 12);
    inv.add_product("Chips", "Salted", 7.5, 5);

    let mut tx = Transactions::new();

    match tx.record_sale("Cola", 3, 12.0, &mut inv) {
        Ok(sale) => println!("Sale: {} x{} = {}", sale.product_name(), sale.quantity(), sale.total_price()),
        Err(e) => println!("Error: {:?}", e),
    }

    match tx.record_purchase("Chips", 10, 5.0, &mut inv) {
        Ok(purchase) => println!("Purchase: {} x{} = {}", purchase.product_name(), purchase.quantity(), purchase.total_cost()),
        Err(e) => println!("Error: {:?}", e),
    }

    println!();
    println!("{}", report_inventory(&inv));
    println!("{}", report_sales(&tx));
    println!("{}", report_purchases(&tx));
}

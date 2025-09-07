use store_inventory_management_system::inventory::{Inventory, report_inventory};
use store_inventory_management_system::transactions::{Transactions, report_sales, report_purchases, StoreError};

#[test]
fn add_and_list_products() {
    let mut inv = Inventory::new();
    inv.add_product("Cola", "330ml can", 10.0, 12);
    inv.add_product("Chips", "Salted", 7.5, 5);
    assert_eq!(inv.all().len(), 2);
    assert_eq!(inv.all()[0].name(), "Cola");
}

#[test]
fn edit_and_delete_by_name() {
    let mut inv = Inventory::new();
    inv.add_product("Cola", "330ml can", 10.0, 12);
    inv.edit_product("Cola", Some("ColaZero"), Some("No sugar"), Some(11.5), Some(10));
    assert!(inv.all().iter().any(|p| p.name() == "ColaZero"));

    inv.del_product("ColaZero");
    assert!(inv.all().is_empty());
}

#[test]
fn record_sale_decreases_stock() {
    let mut inv = Inventory::new();
    inv.add_product("Cola", "330ml can", 10.0, 5);
    let mut tx = Transactions::new();

    let res = tx.record_sale("Cola", 3, 12.0, &mut inv);
    assert!(res.is_ok());
    assert_eq!(inv.all()[0].quantity(), 2);

    // make sure these getters are pub in transactions.rs:
    // pub fn sales(&self) -> &Vec<Sale> { &self.sales }
    assert_eq!(tx.get_sales().len(), 1);
    assert!((tx.total_revenue() - 36.0).abs() < 1e-6);
}

#[test]
fn out_of_stock_error() {
    let mut inv = Inventory::new();
    inv.add_product("Cola", "330ml can", 10.0, 2);
    let mut tx = Transactions::new();

    let res = tx.record_sale("Cola", 3, 12.0, &mut inv);
    match res {
        Err(StoreError::OutOfStock { product, requested, available }) => {
            assert_eq!(product, "Cola");
            assert_eq!(requested, 3);
            assert_eq!(available, 2);
        }
        _ => panic!("expected OutOfStock error"),
    }
}

#[test]
fn record_purchase_increases_stock() {
    let mut inv = Inventory::new();
    inv.add_product("Chips", "Salted", 7.5, 5);
    let mut tx = Transactions::new();

    let res = tx.record_purchase("Chips", 10, 5.0, &mut inv);
    assert!(res.is_ok());
    assert_eq!(inv.all()[0].quantity(), 15);

    // pub fn purchases(&self) -> &Vec<Purchase> { &self.purchases }
    assert_eq!(tx.get_purchases().len(), 1);
    assert!((tx.total_purchase_cost() - 50.0).abs() < 1e-6);
}

#[test]
fn reports_produce_text() {
    let mut inv = Inventory::new();
    inv.add_product("Cola", "330ml can", 10.0, 1);
    let mut tx = Transactions::new();
    let _ = tx.record_purchase("Cola", 1, 8.0, &mut inv);

    assert!(!report_inventory(&inv).is_empty());
    assert!(!report_sales(&tx).is_empty());
    assert!(!report_purchases(&tx).is_empty());
}

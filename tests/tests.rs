use store_inventory_management_system::*; // added missing semicolon

//
// Inventory Tests
//
#[test]
fn add_product_increases_inventory() {
    let mut inv = Inventory::new();
    inv.add_product("Cola", "Sweet drink", 10.0, 50);
    assert_eq!(inv.all().len(), 1); // changed products() to all()
}

#[test]
fn add_multiple_products() {
    let mut inv = Inventory::new();
    inv.add_product("Cola", "Sweet drink", 10.0, 50);
    inv.add_product("Sprite", "Lemon drink", 8.0, 20);
    assert_eq!(inv.all().len(), 2); // changed products() to all()
}

#[test]
fn edit_product_changes_fields_correctly() {
    let mut inv = Inventory::new();
    inv.add_product("Cola", "Sweet drink", 10.0, 50);
    inv.edit_product("Cola", Some("ColaZero"), Some("No sugar"), Some(12.5), None); // added None for quantity parameter

    let p = inv.all().iter().find(|p| p.name() == "ColaZero").unwrap(); // changed products() to all()
    assert_eq!(p.description(), "No sugar");
    assert_eq!(p.price(), 12.5);
}

#[test]
fn edit_nonexistent_product_does_nothing() {
    let mut inv = Inventory::new();
    inv.add_product("Cola", "Sweet drink", 10.0, 50);
    inv.edit_product("Fanta", Some("FantaZero"), None, None, None); // added None for quantity parameter
    assert!(inv.all().iter().all(|p| p.name() != "FantaZero")); // changed products() to all()
}

#[test]
fn delete_product_removes_by_name() {
    let mut inv = Inventory::new();
    inv.add_product("Sprite", "Lemon drink", 8.0, 20);
    inv.del_product("Sprite");
    assert!(inv.all().is_empty()); // changed products() to all()
}

#[test]
fn delete_nonexistent_product_does_nothing() {
    let mut inv = Inventory::new();
    inv.add_product("Sprite", "Lemon drink", 8.0, 20);
    inv.del_product("Fanta");
    assert_eq!(inv.all().len(), 1); // changed products() to all()
}

#[test]
fn add_product_with_zero_quantity() {
    let mut inv = Inventory::new();
    inv.add_product("Empty", "Zero stock", 5.0, 0);
    assert_eq!(inv.all()[0].quantity(), 0); // changed products() to all()
}

#[test]
fn add_product_with_zero_price() {
    let mut inv = Inventory::new();
    inv.add_product("Freebie", "Promotional", 0.0, 10);
    assert_eq!(inv.all()[0].price(), 0.0); // changed products() to all()
}

//
// Sales Tests
//
#[test]
fn record_sale_reduces_quantity_and_adds_transaction() {
    let mut inv = Inventory::new();
    inv.add_product("Pepsi", "Soda", 9.5, 10);
    let mut tx = Transactions::new();

    let _ = tx.record_sale("Pepsi", 2, 9.5, &mut inv); // fixed parameter order
    assert_eq!(inv.all()[0].quantity(), 8); // changed products() to all()
    assert_eq!(tx.get_sales().len(), 1); // changed sales() to get_sales()
    assert!((tx.total_revenue() - 19.0).abs() < f64::EPSILON);
}

#[test]
fn record_sale_out_of_stock_should_not_add_transaction() {
    let mut inv = Inventory::new();
    inv.add_product("Juice", "Orange", 5.0, 1);
    let mut tx = Transactions::new();

    let _ = tx.record_sale("Juice", 5, 5.0, &mut inv); // fixed parameter order and added Result handling
    assert!(tx.get_sales().is_empty()); // changed sales() to get_sales()
    assert_eq!(inv.all()[0].quantity(), 1); // changed products() to all()
}

#[test]
fn record_sale_nonexistent_product_does_nothing() {
    let mut inv = Inventory::new();
    inv.add_product("Water", "Still", 3.0, 10);
    let mut tx = Transactions::new();

    let _ = tx.record_sale("Beer", 2, 3.0, &mut inv); // fixed parameter order and added Result handling
    assert!(tx.get_sales().is_empty()); // changed sales() to get_sales()
}

//
// Purchase Tests
//
#[test]
fn record_purchase_increases_quantity_and_adds_transaction() {
    let mut inv = Inventory::new();
    inv.add_product("Fanta", "Orange soda", 7.0, 5);
    let mut tx = Transactions::new();

    let _ = tx.record_purchase("Fanta", 10, 6.5, &mut inv); // fixed parameter order and added Result handling
    assert_eq!(inv.all()[0].quantity(), 15); // changed products() to all()
    assert_eq!(tx.get_purchases().len(), 1); // changed purchases() to get_purchases()
    assert!((tx.total_purchase_cost() - 65.0).abs() < f64::EPSILON);
}

#[test]
fn record_purchase_nonexistent_product_does_nothing() {
    let mut inv = Inventory::new();
    let mut tx = Transactions::new();

    let _ = tx.record_purchase("NonExistent", 10, 5.0, &mut inv); // fixed parameter order and added Result handling
    assert!(tx.get_purchases().is_empty()); // changed purchases() to get_purchases()
}

//
// Reporting Tests
//
#[test]
fn report_inventory_runs_without_panic() {
    let mut inv = Inventory::new();
    inv.add_product("Milk", "Dairy", 2.0, 5);
    let _ = report_inventory(&inv); // changed to call the standalone function instead of a method
}
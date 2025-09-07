// Integration tests live under `tests/` and use the crate as an external dependency.
use store_inventory_management_system::{Inventory, Transactions, report_inventory, report_sales, report_purchases, StoreError};

#[test]
fn add_edit_delete_product_flow() {
    let mut inv = Inventory::new();

    // add
    inv.add_product("Cola", "330ml can", 10.0, 12);
    inv.add_product("Chips", "Salted", 7.5, 5);
    let inv_report = report_inventory(&inv);
    assert!(inv_report.contains("Cola"));
    assert!(inv_report.contains("Chips"));

    // edit (rename + price + qty)
    inv.edit_product(
        "Cola",
        Some("Cola Zero"),
        Some("330ml can (zero sugar)"),
        Some(11.0),
        Some(20),
    );
    let inv_report = report_inventory(&inv);
    assert!(!inv_report.contains("Cola\n")); // original name gone (newline avoids partial match)
    assert!(inv_report.contains("Cola Zero"));
    assert!(inv_report.contains("11.0"));
    assert!(inv_report.contains("20"));

    // delete
    inv.del_product("Chips");
    let inv_report = report_inventory(&inv);
    assert!(!inv_report.contains("Chips"));
}

#[test]
fn purchase_increases_stock_and_is_recorded() {
    let mut inv = Inventory::new();
    let mut tx = Transactions::new();

    inv.add_product("Bread", "Whole grain", 18.0, 10);

    // supplier purchase
    let p = tx.record_purchase("Bread", 15, 12.0, &mut inv).expect("purchase should work");
    assert_eq!(p.product_name(), "Bread");
    assert_eq!(p.quantity(), 15);
    assert!((p.total_cost() - 15.0 * 12.0).abs() < 1e-6);

    // stock increased
    let rep = report_inventory(&inv);
    // expect quantity 10 + 15 = 25 somewhere
    assert!(rep.contains("25"));

    // recorded in purchases report
    let pr = report_purchases(&tx);
    assert!(pr.contains("Bread"));
    assert!(pr.contains("15"));
}

#[test]
fn sale_decreases_stock_and_is_recorded() {
    let mut inv = Inventory::new();
    let mut tx = Transactions::new();

    inv.add_product("Milk", "1L", 24.0, 8);

    // customer purchase (sale)
    let s = tx.record_sale("Milk", 3, 25.0, &mut inv).expect("sale should work");
    assert_eq!(s.product_name(), "Milk");
    assert_eq!(s.quantity(), 3);
    assert!((s.total_price() - 3.0 * 25.0).abs() < 1e-6);

    // stock decreased: 8 - 3 = 5
    let rep = report_inventory(&inv);
    assert!(rep.contains("Milk"));
    assert!(rep.contains("5"));

    // recorded in sales report
    let sr = report_sales(&tx);
    assert!(sr.contains("Milk"));
    assert!(sr.contains("3"));
}

#[test]
fn cannot_sell_more_than_stock() {
    let mut inv = Inventory::new();
    let mut tx = Transactions::new();

    inv.add_product("Eggs", "Dozen", 65.0, 2);

    // try to sell 5 while stock is 2
    let res = tx.record_sale("Eggs", 5, 70.0, &mut inv);
    assert!(res.is_err(), "should fail when selling more than stock");

    // inventory must remain unchanged (quantity still 2)
    let rep = report_inventory(&inv);
    assert!(rep.contains("Eggs"));
    assert!(rep.contains("2"));

    // and nothing recorded in sales
    let sr = report_sales(&tx);
    assert!(!sr.contains("Eggs"));
}

// Optional: selling a non-existent product should fail gracefully
#[test]
fn cannot_sell_unknown_product() {
    let mut inv = Inventory::new();
    let mut tx = Transactions::new();

    let res = tx.record_sale("Unknown", 1, 10.0, &mut inv);
    assert!(res.is_err());
}

// Optional: purchasing a non-existent product could either create it or fail.
// Adjust based on your implementation. Here we expect failure.
#[test]
fn purchasing_unknown_product_should_fail_unless_implemented_otherwise() {
    let mut inv = Inventory::new();
    let mut tx = Transactions::new();

    let res = tx.record_purchase("NewThing", 5, 3.0, &mut inv);
    assert!(res.is_err(), "If your logic auto-creates products, flip this assertion.");
}

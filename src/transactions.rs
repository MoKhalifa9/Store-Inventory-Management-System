#![allow(dead_code)]
use crate::inventory::Inventory;

#[derive(Debug)]
pub enum StoreError {
    InvalidInput(String),
    OutOfStock { product: String, requested: u32, available: u32 },
    NotFound(String),
}
pub struct Sale {
    product_name: String,
    quantity: u32,
    unit_price: f64,
    total_price: f64,
}

impl Sale {
    pub fn product_name(&self) -> &str { &self.product_name }
    pub fn quantity(&self) -> u32 { self.quantity }
    pub fn unit_price(&self) -> f64 { self.unit_price }
    pub fn total_price(&self) -> f64 { self.total_price }
}


pub struct Purchase {
    product_name: String,
    quantity: u32,
    unit_price: f64,
    total_cost: f64,
}

impl Purchase {
    pub fn product_name(&self) -> &str { &self.product_name }
    pub fn quantity(&self) -> u32 { self.quantity }
    pub fn unit_price(&self) -> f64 { self.unit_price }
    pub fn total_cost(&self) -> f64 { self.total_cost }
}

pub struct Transactions {
    sales: Vec<Sale>,
    purchases: Vec<Purchase>,
}

impl Transactions {
    pub fn new() -> Self {
        Self {
            sales: Vec::new(),
            purchases: Vec::new()
        }
    }

    pub fn record_sale(
        &mut self,
        product_name: &str,
        quantity: u32,
        unit_price: f64,
        inv: &mut Inventory,
    ) -> Result<&Sale, StoreError> {
        if quantity == 0 {
            return Err(StoreError::InvalidInput("Quantity must be > 0".into()));
        }
        if unit_price <= 0.0 {
            return Err(StoreError::InvalidInput("Unit price must be > 0".into()));
        }

        if let Some(p) = inv.find_mut(product_name) {
            if p.quantity() < quantity {
                return Err(StoreError::OutOfStock {
                    product: product_name.to_string(),
                    requested: quantity,
                    available: p.quantity(),
                });
            }
            p.remove_stock(quantity);

            let total_price = quantity as f64 * unit_price;
            self.sales.push(Sale {
                product_name: product_name.to_string(),
                quantity,
                unit_price,
                total_price,
            });
            Ok(self.sales.last().unwrap())
        } else {
            Err(StoreError::NotFound(product_name.to_string()))
        }
    }

    pub fn record_purchase(
        &mut self,
        product_name: &str,
        quantity: u32,
        unit_price: f64,
        inv: &mut Inventory,
    ) -> Result<&Purchase, StoreError> {
        if quantity == 0 {
            return Err(StoreError::InvalidInput("Quantity must be > 0".into()));
        }
        if unit_price <= 0.0 {
            return Err(StoreError::InvalidInput("Unit price must be > 0".into()));
        }

        if let Some(p) = inv.find_mut(product_name) {
            p.add_stock(quantity);

            let total_cost = quantity as f64 * unit_price;
            self.purchases.push(Purchase {
                product_name: product_name.to_string(),
                quantity,
                unit_price,
                total_cost,
            });
            Ok(self.purchases.last().unwrap())
        } else {
            Err(StoreError::NotFound(product_name.to_string()))
        }
    }

    
    pub fn get_sales(&self) -> &Vec<Sale> {
        &self.sales
    }
    
    pub fn get_purchases(&self) -> &[Purchase] {
        &self.purchases
    }

    pub fn total_revenue(&self) -> f64 {
        self.sales.iter().map(|s| s.total_price).sum()
    }

    pub fn total_purchase_cost(&self) -> f64 {
        self.purchases.iter().map(|p| p.total_cost).sum()
    }
}

pub fn report_sales(tx: &Transactions) -> String {
    let mut out = String::new();
    out.push_str("No | Product            | Qty  | Unit   | Total\n");
    out.push_str("-----------------------------------------------\n");

    for (i, s) in tx.sales.iter().enumerate() {
        out.push_str(&format!(
            "{:<2} | {:<18} | {:>3}  | {:>6.2} | {:>6.2}\n",
            i + 1, s.product_name, s.quantity, s.unit_price, s.total_price
        ));
    }
    out
}

pub fn report_purchases(tx: &Transactions) -> String {
    let mut out = String::new();
    out.push_str("No | Product            | Qty  | Unit   | Total\n");
    out.push_str("-----------------------------------------------\n");

    for (i, p) in tx.purchases.iter().enumerate() {
        out.push_str(&format!(
            "{:<2} | {:<18} | {:>3}  | {:>6.2} | {:>6.2}\n",
            i + 1, p.product_name, p.quantity, p.unit_price, p.total_cost
        ));
    }
    out
}


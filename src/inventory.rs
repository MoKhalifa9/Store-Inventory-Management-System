#![allow(dead_code)]
pub struct Product {
    name: String,
    description: String,
    price: f64,
    quantity: u32,
}

impl Product {
    pub fn new(name: &str, description: &str, price: f64, quantity: u32) -> Self {
        let price = if price < 0.0 { 0.0 } else { price };
        let quantity = quantity.max(0);
        Self {
            name: name.to_string(),
            description: description.to_string(),
            price,
            quantity,
        }
    }

    // ---- GETTERS ----
    pub fn name(&self) -> &str { &self.name }
    pub fn description(&self) -> &str { &self.description }
    pub fn price(&self) -> f64 { self.price }
    pub fn quantity(&self) -> u32 { self.quantity }

    // ---- SETTERS ----
    pub fn set_description(&mut self, description: &str) { self.description = description.to_string();}
    pub fn set_price(&mut self, price: f64) { if price >= 0.0 { self.price = price; } }
    pub fn set_quantity(&mut self, quantity: u32) { self.quantity = quantity;}
    pub fn add_stock(&mut self, qty: u32) { self.quantity += qty;}
    pub fn remove_stock(&mut self, qty: u32) {
        if self.quantity >= qty {
            self.quantity -= qty;
        } else {
            self.quantity = 0; // clamp at 0
        }
    }
}

pub struct Inventory {
    products: Vec<Product>,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory { products: Vec::new() }
    }
    pub fn add_product(
        &mut self,
        name: &str,
        description: &str,
        price: f64,
        quantity: u32,
    ) {
        let product = Product {
            name: name.to_string(),
            description: description.to_string(),
            price,
            quantity,
        };
        self.products.push(product);
    }

    pub fn edit_product(
        &mut self,
        index: usize,
        name: Option<String>,
        description: Option<String>,
        price: Option<f64>,
        quantity: Option<u32>,
    ) {
        if let Some(product) = self.products.get_mut(index) {
            if let Some(name) = name { product.name = name;}
            if let Some(description) = description { product.description = description;}
            if let Some(price) = price { product.price = price; }
            if let Some(quantity) = quantity { product.quantity = quantity; }
        }
    }

    pub fn del_product(&mut self, index: usize) {
        if index < self.products.len() {
            self.products.remove(index);
        }
    }

    pub fn all(&self) -> &Vec<Product> {
        &self.products
    }

    pub fn find_mut(&mut self, name: &str) -> Option<&mut Product> {
        self.products.iter_mut().find(|p| p.name() == name)
    }
}

pub fn report_inventory(inv: &Inventory) -> String {
    let mut out = String::new();
    out.push_str("Name                 | Description              | Price   | Qty\n");
    out.push_str("---------------------------------------------------------------\n");

    for p in inv.all() {
        out.push_str(&format!(
            "{:<20} | {:<24} | {:>7.2} | {:>3}\n",
            p.name, p.description, p.price, p.quantity
        ));
    }
    out
}
pub struct Sales {
    product_name: String,
    quantity: u32,
    unit_price: f64,
    total_price: f64,
}

pub struct Purchase {
    product_name: String,
    quantity: u32,
    unit_price: f64,
    total_cost: f64,
}

pub struct Transactions {
    sales: Vec<Sales>,
    purchases: Vec<Purchase>,
}

impl Transactions {
    pub fn new() -> Self {
        Self {
            sales: Vec::new(),
            purchases: Vec::new(),
        }
    }

    pub fn record_sale(&mut self, product_name: String, quantity: u32, unit_price: f64) {
        let total_price = quantity as f64 * unit_price;
        let sale = Sales {
            product_name,
            quantity,
            unit_price,
            total_price,
        };
        self.sales.push(sale);
    }

    pub fn record_purchase(&mut self, product_name: String, quantity: u32, unit_price: f64) -> &Purchase {
        let total_cost = quantity as f64 * unit_price;
        self.purchases.push(Purchase {
            product_name: product_name.to_string(),
            quantity,
            unit_price,
            total_cost,
        });
        self.purchases.last().unwrap()
    }

    
    pub fn get_sales(&self) -> &Vec<Sales> {
        &self.sales
    }
    
    pub fn get_purchases(&self) -> &[Purchase] {
        &self.purchases
    }
}
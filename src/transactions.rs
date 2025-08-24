struct Sales {
    product_name: String,
    quantity: u32,
    unit_price: f64,
    total_price: f64,
}

pub struct Transactions {
    sales: Vec<Sales>,
}

impl Transactions {
    pub fn new() -> Self {
        Transactions { sales: Vec::new() }
    }

    pub fn record_sale(&mut self, product_name: &str, quantity: u32, unit_price: f64) {
        let total_price = quantity as f64 * unit_price;
        let sale = Sales {
            product_name,
            quantity,
            unit_price,
            total_price,
        };
        self.sales.push(sale);
    }

    pub fn get_sales(&self) -> &Vec<Sales> {
        &self.sales
    }
}
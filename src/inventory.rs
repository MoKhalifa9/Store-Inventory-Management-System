struct Product {
    name: String,
    description: String,
    price: f64,
    quantity: u32,
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
        name: String,
        description: String,
        price: f64,
        quantity: u32,
    ) {
        let product = Product {
            name,
            description,
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
            if let Some(name) = name {
                product.name = name;
            }
            if let Some(description) = description {
                product.description = description;
            }
            if let Some(price) = price {
                product.price = price;
            }
            if let Some(quantity) = quantity {
                product.quantity = quantity;
            }
        }
    }

    pub fn del_product(&mut self, index: usize) {
        if index < self.products.len() {
            self.products.remove(index);
        }
    }
}
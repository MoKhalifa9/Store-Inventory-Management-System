# 🦀 Rusty Store Inventory Management System

A simple text-based inventory management system written in Rust.
It manages products, sales, and purchases, with basic reporting and authentication.
This project was built for learning purposes.

## ✨ Features
- Inventory Management
  - Add, edit, and delete products (name, description, price, quantity).
- Sales Management
  - Record sales and automatically reduce stock.
  - Calculate revenue from transactions.
- Purchase Management
  - Record purchases and increase stock.
  - Calculate total purchase costs.
- Reporting
  - Display inventory, sales, and purchase history in a clear text format.
- Authentication
  - Hardcoded username and password:
    - Username: admin
    - Password: secret
  - Optional: override password with the environment variable APP_PASSWORD.
- Error Handling
  - Prevents out-of-stock sales.
  - Ignores operations on non-existent products.
- Testing
  - Comprehensive integration tests under tests/
  - Covers all major use cases and edge cases

## 📂 Project Structure
```text
.
├── Users.txt
├── Cargo.toml
├── src/
│ ├── lib.rs # Module exports
│ ├── main.rs # CLI menu (text-based)
│ ├── inventory.rs # Inventory logic
│ ├── transactions.rs # Sales & purchase logic
│ └── security.rs # Authentication
└── tests/
│ └── tests.rs # Integration tests
```

## 🚀 Getting Started
### Pre-Requisites
  - Install [Rust](https://www.rust-lang.org/tools/install)
### Clone The Repository
```bash
git clone https://github.com/MoKhalifa9/Store-Inventory-Management-System.git
```
### Build & Run
```bash
cargo build
cargo run
```
### Run Tests
```bash
cargo test
```

## 📜 Notes
- This is a personal learning project (not production-ready).
- Focuses on Rust basics: ownership, borrowing, structs, error handling, and modular design.
- No GUI — this is a text-based CLI project.

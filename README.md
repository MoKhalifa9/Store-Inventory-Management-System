# Rusty Store Inventory Management System

A simple **text-based inventory management system** written in Rust.  
This project manages a store’s **inventory, sales, purchases, reports**, and adds **basic authentication** with different user roles.

---

## Features

- **User Authentication**
  - Accounts are stored in a simple `users.txt` file.
  - Default bootstrap admin:  
    ```
    username: admin
    password: secret
    role: Admin
    ```
  - Clients can sign up directly.
  - Users can change their own password.
  - Roles:
    - **Client** → view products & purchase
    - **Manager** → add, edit, delete products & record purchases
    - **Admin** → all manager features + view sales & purchase reports

- **Inventory Management**
  - Add, edit, delete products
  - Each product has name, description, price, and quantity
  - Inventory starts with some seeded items:
    - Cola
    - Chips
    - Bread

- **Sales & Purchases**
  - Record sales with product, quantity, and unit price
  - Record purchases (restock) with product, quantity, and cost

- **Reports**
  - Admin can view:
    - Inventory list
    - Sales history
    - Purchase history

- **Error Handling**
  - Prevents selling items that don’t exist or insufficient stock
  - Handles invalid input gracefully

---

## Project Structure

```
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

---

## How to Run
1. Pre-Requisites
  - Install [Rust](https://www.rust-lang.org/tools/install)
2. Clone the repo and build:
   ```bash
   git clone https://github.com/MoKhalifa9/Store-Inventory-Management-System.git
   ```
   ```bash
   cargo build
   ```

3. Run the program:
   ```bash
   cargo run
   ```

4. On first run, a default admin account is created automatically:
   ```
   admin / secret
   ```

---

## Usage

- From the welcome screen:
  - **Sign up** → create a new Client account
  - **Log in** → enter username and password
- Depending on role:
  - **Client Menu**
    - View products
    - Purchase products
    - Change password
  - **Manager Menu**
    - All client options
    - Add, edit, delete products
    - Record supplier purchases
    - Change password
  - **Admin Menu**
    - All manager options
    - View inventory, sales, and purchase reports
    - Change password

---

## Roles & Permissions

| Capability              | Client | Manager | Admin |
|:--------------------------|:--------:|:---------:|:-------:|
| View products            | ✅     | ✅      | ✅    |
| Purchase products        | ✅     | ✅      | ✅    |
| Add/edit/delete products | ❌     | ✅      | ✅    |
| Record supplier purchase | ❌     | ✅      | ✅    |
| Change password          | ✅     | ✅      | ✅    |
| View reports             | ❌     | ❌      | ✅    |

---
## Promoting to Manager

- By default, all signups are created as `Client`.  
- To make someone a `Manager`, open `users.txt` and change their role from `Client` to `Manager`.  
- Example:  
  ```
  username,password,Manager
  ```
- Next time they log in, they’ll see the **Manager menu**.

---
## 📜 Notes
- This is a personal learning project (not production-ready).
- Focuses on Rust basics: ownership, borrowing, structs, error handling, and modular design.
- No GUI — this is a text-based CLI project.

---
## Example Run

```
== Welcome ==
1) Log in
2) Sign up (Client)
0) Exit
> 1
Username: admin
Password: secret
Logged in as Admin.

== Manager Menu ==
1) View products
2) Add product
3) Edit product
4) Delete product
5) Record supplier purchase
6) View reports
7) Change password
0) Logout
```

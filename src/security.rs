use std::fs::{self, OpenOptions};
use std::io::{self, Write, BufRead, BufReader};

#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    Client,
    Manager,
    Admin,
}

#[derive(Debug, Clone)]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: Role,
}

fn parse_role(s: &str) -> Role {
    match s {
        "Admin" => Role::Admin,
        "Manager" => Role::Manager,
        _ => Role::Client,
    }
}

fn role_to_str(r: &Role) -> &'static str {
    match r {
        Role::Admin => "Admin",
        Role::Manager => "Manager",
        Role::Client => "Client",
    }
}

fn load_users() -> Vec<User> {
    let path = "users.txt";
    if !std::path::Path::new(path).exists() {
        let _ = fs::write(path, "admin,secret,Admin\n");
    }
    let file = fs::File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader.lines()
        .filter_map(|line| {
            if let Ok(l) = line {
                let parts: Vec<&str> = l.split(',').collect();
                if parts.len() == 3 {
                    Some(User {
                        username: parts[0].to_string(),
                        password: parts[1].to_string(),
                        role: parse_role(parts[2]),
                    })
                } else { None }
            } else { None }
        })
        .collect()
}

fn save_users(users: &[User]) {
    let mut data = String::new();
    for u in users {
        data.push_str(&format!("{},{},{}\n", u.username, u.password, role_to_str(&u.role)));
    }
    let _ = fs::write("users.txt", data);
}

fn prompt(label: &str) -> String {
    print!("{label}");
    let _ = io::stdout().flush();
    let mut s = String::new();
    let _ = io::stdin().read_line(&mut s);
    s.trim().to_string()
}

pub fn signup() {
    let mut users = load_users();
    let username = prompt("Choose username: ");
    if users.iter().any(|u| u.username == username) {
        println!("Username already exists!");
        return;
    }
    let password = prompt("Choose password: ");
    users.push(User { username, password, role: Role::Client });
    save_users(&users);
    println!("Account created! You can now log in.");
}

pub fn login() -> Option<User> {
    let users = load_users();
    let username = prompt("Username: ");
    let password = prompt("Password: ");
    for u in users {
        if u.username == username && u.password == password {
            println!("Logged in as {:?}.", u.role);
            return Some(u);
        }
    }
    println!("Invalid credentials.");
    None
}

pub fn change_password(current_user: &User) {
    let mut users = load_users();
    let old = prompt("Old password: ");
    if old != current_user.password {
        println!("Wrong password.");
        return;
    }
    let newp = prompt("New password: ");
    for u in users.iter_mut() {
        if u.username == current_user.username {
            u.password = newp.clone();
        }
    }
    save_users(&users);
    println!("Password changed!");
}
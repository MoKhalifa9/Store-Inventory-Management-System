use std::io::{self, Write};
use std::thread;
use std::time::Duration;

const MANAGER_USER: &str = "admin";
const DEFAULT_PASS: &str = "secret";
const MAX_ATTEMPTS: u8 = 3;

fn prompt(label: &str) -> String {
    print!("{label}");
    let _ = io::stdout().flush();
    let mut s = String::new();
    let _ = io::stdin().read_line(&mut s);
    s.trim().to_string()
}

fn check_credentials(username: &str, password: &str) -> bool {
    let env_pass = std::env::var("APP_PASSWORD").unwrap_or_else(|_| DEFAULT_PASS.to_string());
    username == MANAGER_USER && password == env_pass
}

pub fn require_login() -> bool {
    let mut attempt: u8 = 0;
    let mut delay_secs = 1u64;

    loop {
        attempt += 1;

        let username = prompt("Username: ");
        let password = prompt("Password: ");

        if check_credentials(&username, &password) {
            println!("Access granted");
            return true;
        } else {
            eprintln!("Invalid credentials ({}/{})", attempt, MAX_ATTEMPTS);
        }

        if attempt >= MAX_ATTEMPTS {
            eprintln!("Too many failed attempts. Exiting.");
            return false;
        }

        std::thread::sleep(Duration::from_secs(delay_secs));
        delay_secs = (delay_secs * 2).min(4);
    }
}

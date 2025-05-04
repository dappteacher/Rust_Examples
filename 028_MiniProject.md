# 🦀 Mini Rust Project: Token Wallet CLI Simulator

A simple command-line wallet simulation to help you reinforce core Rust concepts such as:

* ✅ Structs, Traits, and Match logic
* ✅ Ownership & Borrowing
* ✅ File I/O
* ✅ Serialization/Deserialization with `serde`
* ✅ Command-line input handling

---

## 🎯 Project Overview

This CLI app allows users to:

* Create a wallet
* Deposit and withdraw tokens
* View wallet balance
* Save wallet state to a JSON file
* Load wallet state from a file

---

## 📁 Project Structure

```
token_wallet_cli/
├── src/
│   └── main.rs
├── Cargo.toml
```

---

## 📦 Dependencies (`Cargo.toml`)

```toml
[package]
name = "token_wallet_cli"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

---

## 🧠 Code (`main.rs`)

```rust
use std::fs::{File, OpenOptions};
use std::io::{self, Write, BufReader};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Wallet {
    owner: String,
    balance: u64,
}

impl Wallet {
    fn new(owner: String) -> Self {
        Wallet { owner, balance: 0 }
    }

    fn deposit(&mut self, amount: u64) {
        self.balance += amount;
        println!("✅ Deposited {} tokens.", amount);
    }

    fn withdraw(&mut self, amount: u64) {
        if self.balance >= amount {
            self.balance -= amount;
            println!("✅ Withdrew {} tokens.", amount);
        } else {
            println!("❌ Insufficient balance.");
        }
    }

    fn display(&self) {
        println!("👤 Owner: {}", self.owner);
        println!("💰 Balance: {} tokens", self.balance);
    }

    fn save_to_file(&self, filename: &str) {
        let json = serde_json::to_string_pretty(self).expect("Serialization failed");
        let mut file = File::create(filename).expect("Unable to create file");
        file.write_all(json.as_bytes()).expect("Write failed");
        println!("💾 Wallet saved to {}", filename);
    }

    fn load_from_file(filename: &str) -> Option<Self> {
        let file = OpenOptions::new().read(true).open(filename).ok()?;
        let reader = BufReader::new(file);
        let wallet = serde_json::from_reader(reader).ok()?;
        println!("📂 Wallet loaded from {}", filename);
        Some(wallet)
    }
}

fn main() {
    println!("🚀 Welcome to the Token Wallet CLI!");
    println!("1. Create a new wallet");
    println!("2. Load an existing wallet");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let choice = input.trim();

    let mut wallet = match choice {
        "1" => {
            println!("Enter wallet owner name:");
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            Wallet::new(input.trim().to_string())
        }
        "2" => {
            println!("Enter filename to load:");
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let filename = input.trim();
            Wallet::load_from_file(filename).unwrap_or_else(|| {
                println!("⚠️ Failed to load wallet. Creating a new one.");
                Wallet::new("Unknown".to_string())
            })
        }
        _ => {
            println!("❌ Invalid choice.");
            return;
        }
    };

    loop {
        println!("\n📋 Menu:");
        println!("1. Deposit");
        println!("2. Withdraw");
        println!("3. Check Balance");
        println!("4. Save Wallet");
        println!("5. Exit");

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "1" => {
                println!("Enter amount to deposit:");
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                if let Ok(amount) = input.trim().parse::<u64>() {
                    wallet.deposit(amount);
                } else {
                    println!("❌ Invalid amount.");
                }
            }
            "2" => {
                println!("Enter amount to withdraw:");
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                if let Ok(amount) = input.trim().parse::<u64>() {
                    wallet.withdraw(amount);
                } else {
                    println!("❌ Invalid amount.");
                }
            }
            "3" => wallet.display(),
            "4" => {
                println!("Enter filename to save:");
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                wallet.save_to_file(input.trim());
            }
            "5" => {
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("❌ Invalid option."),
        }
    }
}
```

---

## 🧪 How to Run

```bash
cargo new token_wallet_cli
cd token_wallet_cli
# Replace the contents of src/main.rs and Cargo.toml with the above
cargo run
```

---

## 🧰 Concepts Practiced

| Concept               | Usage Example                        |
| --------------------- | ------------------------------------ |
| `Struct`              | `Wallet` structure for user accounts |
| `Traits`              | `Serialize`, `Deserialize` for JSON  |
| Ownership & Borrowing | Managed with user input and state    |
| File I/O              | JSON-based save/load from disk       |
| Control Flow          | Menu system using `match`            |
| CLI Interaction       | `std::io` for terminal-based I/O     |

---

## 🧠 Bonus Ideas

* Add authentication using a PIN or password
* Support multiple wallets
* Add a transaction history log
* Improve UX with colored CLI output (`colored` crate)

---

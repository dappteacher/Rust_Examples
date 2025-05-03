# 🎲 Generating Random Numbers in Rust

Rust does not include random number generation in the standard library by default. Instead, we use the external crate `rand`.

---

## 🔧 Step 1: Add `rand` to `Cargo.toml`

```toml
[dependencies]
rand = "0.8"
```

This adds the `rand` crate, which provides random number generation tools.

---

## 🔢 Generating a Random Number

```rust
use rand::Rng;

fn main() {
    let random_number = rand::thread_rng().gen_range(1..=100);
    println!("Random number: {}", random_number);
}
```

* `rand::thread_rng()` gives you a random number generator local to the current thread.
* `.gen_range(1..=100)` generates a number between **1 and 100**, **inclusive**.

---

## ✅ Full Example: Guess the Number Game

This is a **classic beginner example** in Rust that demonstrates:

* Random number generation
* User input handling
* Match statements
* Loops and conditionals

```rust
use std::cmp::Ordering;
use rand::Rng;
use std::io;

fn main() {
    println!("🎯 Guess a number between 1 and 100!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // Uncomment the next line to cheat 😄
    // println!("(Debug) Secret number is: {}", secret_number);

    loop {
        println!("🔢 Please enter your guess:");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("❌ Invalid input. Please enter a number.");
                continue;
            }
        };

        println!("👉 You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("📉 Too low!"),
            Ordering::Greater => println!("📈 Too high!"),
            Ordering::Equal => {
                println!("🎉 Correct! You win!");
                break;
            }
        }
    }
}
```

---

## 🧠 What You Learned

| Concept                   | Usage in Example                                 |
| ------------------------- | ------------------------------------------------ |
| `rand::Rng` trait         | Generates random numbers                         |
| `.gen_range(start..=end)` | Picks a random number in a range                 |
| `match` statement         | Handles result of comparison and input parsing   |
| `loop`                    | Repeats the guessing until the player is correct |
| `Ordering` enum           | Compares guessed number to secret number         |

---

## 🔐 Why `rand` Is External?

Rust's core is focused on safety and simplicity. To avoid bloating the standard library, functionality like randomness, networking, etc., is provided via **external crates**.

---

## ✅ Summary

* Use the `rand` crate for random numbers.
* Use `thread_rng().gen_range(...)` for easy generation.
* You can use this to build games, simulations, or randomized algorithms.

---

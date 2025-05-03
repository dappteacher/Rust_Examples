# ❌ Error Handling in Rust

Rust encourages **safe and robust error handling** without using exceptions. Instead, it uses `Result` and `Option` types to explicitly handle failures and the absence of values.

---

## 🚨 Two Categories of Errors

| Type               | Description                                   | Common Handling |
|--------------------|-----------------------------------------------|-----------------|
| **Recoverable**    | Expected failures (e.g., file not found)       | `Result<T, E>`  |
| **Unrecoverable**  | Bugs in code (e.g., index out of bounds)       | `panic!()`      |

---

## 🛠️ Using `Result` for Recoverable Errors

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_file_contents() -> Result<String, io::Error> {
    let mut file = File::open("hello.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
````

### ✅ Explanation

* `?` propagates the error to the caller if one occurs.
* The function returns a `Result<String, io::Error>`.

---

## 🔁 Matching on `Result`

```rust
let result = read_file_contents();

match result {
    Ok(text) => println!("File content:\n{text}"),
    Err(e) => eprintln!("Error reading file: {e}"),
}
```

Or use `if let`:

```rust
if let Ok(text) = read_file_contents() {
    println!("File: {text}");
}
```

---

## ⚠️ `Option` for Absence of Value

```rust
fn get_char_at(s: &str, index: usize) -> Option<char> {
    s.chars().nth(index)
}

match get_char_at("Hello", 1) {
    Some(ch) => println!("Char: {ch}"),
    None => println!("No character at that index."),
}
```

---

## 🚨 Unrecoverable Errors: `panic!`

```rust
fn crash() {
    panic!("Something went terribly wrong!");
}
```

Use `panic!()` for bugs and unrecoverable states (e.g., assert failures during development). **Avoid in production code** where possible.

---

## 🔗 Chaining and `?` Operator

```rust
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = read_to_string("file.txt")?;
    println!("Content:\n{content}");
    Ok(())
}
```

* `Box<dyn Error>` allows any error type.
* Use `?` to reduce nesting and simplify error propagation.

---

## 💡 Custom Error Types

```rust
use std::fmt;

#[derive(Debug)]
struct MyError;

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A custom error occurred!")
    }
}

impl std::error::Error for MyError {}
```

You can implement `std::error::Error` for custom error handling.

---

## ✅ Best Practices

* Use `Result<T, E>` for functions that might fail.
* Use `?` to simplify propagation.
* Prefer `Option` when the only issue is "nothing found."
* Avoid using `panic!()` unless absolutely necessary.
* Define custom error types for clarity in larger applications.

---

## 🧠 Summary Table

| Pattern          | Use Case                   |
| ---------------- | -------------------------- |
| `Result<T, E>`   | Recoverable errors         |
| `Option<T>`      | Value may be absent        |
| `?`              | Propagate errors concisely |
| `panic!()`       | Unrecoverable crash        |
| `Box<dyn Error>` | Return any error           |

---

## 🧪 Example: Safe User Input with `Result`

```rust
use std::io;

fn get_user_input() -> Result<i32, String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|_| "Failed to read input".to_string())?;
    input.trim().parse::<i32>().map_err(|_| "Invalid number".to_string())
}

fn main() {
    match get_user_input() {
        Ok(num) => println!("You entered: {num}"),
        Err(e) => println!("Error: {e}"),
    }
}
```

---

📚 Mastering error handling is crucial in Rust — it ensures your applications are **safe**, **predictable**, and **robust**.

---

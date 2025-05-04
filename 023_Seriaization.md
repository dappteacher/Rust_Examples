# 📦 Serialization and Deserialization in Rust with `serde`

Serialization is the process of converting data structures or objects into a format that can be stored or transmitted and reconstructed later. In Rust, the most popular and powerful way to handle this is with the [`serde`](https://serde.rs) crate.

This guide walks you through how to **serialize** and **deserialize** data using JSON as an example format.

---

## 📚 What is `serde`?

> `serde` stands for **Ser**ialize and **De**serialize.

It is a powerful, generic framework for serializing Rust data structures into and from formats like JSON, YAML, TOML, MessagePack, etc.

---

## 🔧 1. Add Dependencies

Add the following lines to your `Cargo.toml`:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
````

---

## 🔷 2. Define Structs and Derive Traits

Use `#[derive(Serialize, Deserialize)]` on your structs or enums.

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u32,
    is_programmer: bool,
}
```

---

## 🔁 3. Serialization (Rust → JSON)

Use `serde_json::to_string()` or `to_string_pretty()`:

```rust
use serde_json;

fn main() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        is_programmer: true,
    };

    let json = serde_json::to_string(&person).expect("Serialization failed");
    println!("JSON: {}", json);
}
```

### Output:

```json
{"name":"Alice","age":30,"is_programmer":true}
```

---

## 🔁 4. Deserialization (JSON → Rust)

Use `serde_json::from_str()`:

```rust
fn main() {
    let json_data = r#"{"name":"Bob","age":25,"is_programmer":false}"#;

    let person: Person = serde_json::from_str(json_data).expect("Deserialization failed");
    println!("Person: {:?}", person);
}
```

---

## 📂 5. Read/Write JSON to File

```rust
use std::fs::File;
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    let person = Person {
        name: "Carol".to_string(),
        age: 28,
        is_programmer: true,
    };

    // Write to file
    let mut file = File::create("person.json")?;
    let json = serde_json::to_string_pretty(&person).unwrap();
    file.write_all(json.as_bytes())?;

    // Read from file
    let mut file = File::open("person.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let deserialized: Person = serde_json::from_str(&contents).unwrap();
    println!("Deserialized: {:?}", deserialized);

    Ok(())
}
```

---

## 📦 Optional Fields and Custom Names

You can customize field names and add default/optional behavior:

```rust
#[derive(Serialize, Deserialize, Debug)]
struct Config {
    #[serde(rename = "user_name")]
    username: String,

    #[serde(default)]
    theme: String, // Defaults to empty string if missing
}
```

---

## 🧪 Complete Example

```rust
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Book {
    title: String,
    author: String,
    pages: u32,
    #[serde(default)]
    published: Option<u16>,
}

fn main() {
    let json_str = r#"
        {
            "title": "The Rust Book",
            "author": "Rust Team",
            "pages": 552
        }
    "#;

    let book: Book = serde_json::from_str(json_str).unwrap();
    println!("{:?}", book);

    let serialized = serde_json::to_string_pretty(&book).unwrap();
    println!("{}", serialized);
}
```

---

## 🌍 Other Supported Formats

By changing the crate, you can easily serialize to other formats:

| Format      | Crate        |
| ----------- | ------------ |
| JSON        | `serde_json` |
| YAML        | `serde_yaml` |
| TOML        | `toml`       |
| MessagePack | `rmp-serde`  |
| CBOR        | `serde_cbor` |

---

## 🛠️ Error Handling

Always handle parsing errors with `Result`:

```rust
match serde_json::from_str::<Person>(&input) {
    Ok(p) => println!("Valid: {:?}", p),
    Err(e) => eprintln!("Invalid JSON: {}", e),
}
```

---

## ✅ Summary

| Task                  | Method                     |
| --------------------- | -------------------------- |
| Serialize             | `serde_json::to_string()`  |
| Deserialize           | `serde_json::from_str()`   |
| File I/O              | Use `std::fs` + JSON       |
| Customize field names | `#[serde(rename = "...")]` |
| Add default values    | `#[serde(default)]`        |

---

## 📎 Further Reading

* [`serde` official website](https://serde.rs)
* [serde\_json on docs.rs](https://docs.rs/serde_json)
* [The Rust Book – Serialization](https://doc.rust-lang.org/book/ch20-02-multithreaded.html)

---

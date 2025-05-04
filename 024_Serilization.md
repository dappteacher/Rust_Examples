# ⚙️ Borsh Serialization in Rust

**Borsh** stands for **Binary Object Representation Serializer for Hashing**. It's a binary serialization format designed to be:

- **Deterministic**
- **Efficient**
- **Simple**
- **Cross-platform**

It is widely used in blockchain ecosystems, especially in the [NEAR Protocol](https://near.org/).

---

## 📦 Why Use Borsh?

- Faster than JSON or other text-based formats.
- Fixed output for the same input (great for hashing and Merkle trees).
- Suitable for smart contracts or systems where binary compactness and determinism matter.

---

## 🔧 Add Dependencies

Add the following to your `Cargo.toml`:

```toml
[dependencies]
borsh = "1.4"
borsh-derive = "1.4"
````

---

## 🧱 Define Your Struct

Use the `#[derive(BorshSerialize, BorshDeserialize)]` macros to enable serialization:

```rust
use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct User {
    username: String,
    age: u8,
    active: bool,
}
```

---

## 💾 Serialization

Convert a struct into bytes:

```rust
fn main() {
    let user = User {
        username: "Jacob".to_string(),
        age: 32,
        active: true,
    };

    let serialized_data = user.try_to_vec().expect("Serialization failed");
    println!("Serialized bytes: {:?}", serialized_data);
}
```

---

## 📂 Deserialization

Convert binary data back into a struct:

```rust
fn main() {
    let user = User {
        username: "Alice".to_string(),
        age: 28,
        active: false,
    };

    let bytes = user.try_to_vec().unwrap();

    let deserialized_user = User::try_from_slice(&bytes).expect("Deserialization failed");

    println!("Deserialized struct: {:?}", deserialized_user);
}
```

---

## 📁 File I/O Example

Writing and reading a serialized struct from a file:

```rust
use std::fs::File;
use std::io::{Write, Read};
use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct Book {
    title: String,
    pages: u32,
}

fn main() -> std::io::Result<()> {
    let book = Book {
        title: "Rust By Example".to_string(),
        pages: 350,
    };

    // Serialize and write to file
    let mut file = File::create("book.borsh")?;
    let encoded = book.try_to_vec().unwrap();
    file.write_all(&encoded)?;

    // Read and deserialize from file
    let mut file = File::open("book.borsh")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let loaded_book = Book::try_from_slice(&buffer).unwrap();
    println!("Loaded from file: {:?}", loaded_book);

    Ok(())
}
```

---

## ⚠️ Notes and Tips

* Always handle errors using `.unwrap()` or better: `?` operator or `match`.
* Structs must have a stable and deterministic order of fields for Borsh to work properly.
* Borsh does **not** support maps (`HashMap`, `BTreeMap`) by default — you must implement custom serialization if needed.

---

## 🛠️ Trait Reference

| Trait              | Description                    |
| ------------------ | ------------------------------ |
| `BorshSerialize`   | For serializing to `Vec<u8>`   |
| `BorshDeserialize` | For deserializing from `&[u8]` |

---

## ✅ Summary

| Operation     | Method                    |
| ------------- | ------------------------- |
| Serialize     | `try_to_vec()`            |
| Deserialize   | `try_from_slice(&[u8])`   |
| Write to file | Use `std::fs::File`       |
| Binary format | Compact and deterministic |

---

## 📚 Further Reading

* [Borsh GitHub Repo](https://github.com/near/borsh)
* [NEAR Protocol Serialization Standard](https://docs.near.org/docs/develop/contracts/serialization)
* [serde vs borsh Comparison](https://docs.rs/borsh/latest/borsh/)

---

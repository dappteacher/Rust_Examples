# 📌 Reference and Borrowing in Rust

In Rust, *ownership* ensures memory safety without needing a garbage collector. One core part of ownership is the concept of **borrowing** through **references**.

---

## 🔁 What Is Borrowing?

**Borrowing** is when you give access to data without giving up ownership. You do this by passing a **reference**.

Rust enforces rules that prevent data races and unsafe memory access by carefully managing these references.

---

## 🧷 Mutable vs Immutable References

- `&T` → **Immutable reference** (read-only access)
- `&mut T` → **Mutable reference** (read and write access)

### ✅ Example: Immutable Reference

```rust
fn main() {
    let name = String::from("Alice");
    greet(&name);
    println!("Name is still usable: {}", name);
}

fn greet(name: &String) {
    println!("Hello, {}!", name);
}
````

✔ The `greet` function borrows `name` immutably — it can read it but not change it.

---

### ✅ Example: Mutable Reference

```rust
fn main() {
    let mut name = String::from("Alice");
    append_lastname(&mut name);
    println!("Updated name: {}", name);
}

fn append_lastname(name: &mut String) {
    name.push_str(" Smith");
}
```

✔ Here, `name` is borrowed mutably, allowing the function to modify it.

---

## 🚫 Borrowing Rules

Rust enforces strict rules to ensure safety:

| Rule                                                                       | Example Violation                          |
| -------------------------------------------------------------------------- | ------------------------------------------ |
| You can have **one** mutable reference OR **any number** of immutable ones | Two mutable references = ❌                 |
| References must always be valid (no dangling references)                   | Return a reference to a local variable = ❌ |

---

## ⚠️ Dangling Reference Example (Won't Compile)

```rust
fn dangle() -> &String {
    let s = String::from("Rust");
    &s // ❌ Returns a reference to memory that gets freed
}
```

Instead, return the actual `String`:

```rust
fn no_dangle() -> String {
    let s = String::from("Rust");
    s // ✅ Ownership is moved
}
```

---

## 🧪 Use Case: Calculate Length Using Borrowing

```rust
fn main() {
    let word = String::from("borrowing");
    let len = calculate_length(&word);
    println!("Length of '{}' is {}", word, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

✔ `calculate_length` borrows the `String` without taking ownership.

---

## 🧠 Summary

| Concept                | Description                             |
| ---------------------- | --------------------------------------- |
| `&T`                   | Immutable reference                     |
| `&mut T`               | Mutable reference                       |
| Borrowing              | Accessing data without taking ownership |
| Borrowing rules        | Prevent unsafe access and data races    |
| No dangling references | Rust ensures references remain valid    |

---

## ✅ Best Practices

* Prefer immutable references unless mutation is required.
* Use borrowing to avoid unnecessary cloning or ownership transfers.
* Be mindful of lifetimes when working with complex references.

---

📘 Rust's borrow checker can feel strict at first, but it's one of the reasons why Rust code is so reliable and memory-safe.

---

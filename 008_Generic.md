# 🌱 Generics in Rust

**Generics** allow you to write **flexible and reusable code** that works with different types without duplicating logic.

Instead of hardcoding types, generics let you write functions, structs, enums, and traits that work on **any data type**.

---

## 🧠 Why Use Generics?

Without generics, you’d need to repeat code like this:

```rust
fn print_i32(x: i32) {
    println!("{}", x);
}

fn print_str(x: &str) {
    println!("{}", x);
}
```

With generics, you write **one version** that works with **any type**:

```rust
fn print_any<T: std::fmt::Display>(x: T) {
    println!("{}", x);
}
```

---

## 🔤 Basic Generic Function

```rust
fn identity<T>(value: T) -> T {
    value
}

fn main() {
    let int_val = identity(10);
    let str_val = identity("hello");

    println!("Int: {}, Str: {}", int_val, str_val);
}
```

> ✅ The function `identity` works with **any type** `T`.

---

## 🧱 Generic Structs

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1.5, y: 2.8 };

    println!("p1: ({}, {})", p1.x, p1.y);
    println!("p2: ({}, {})", p2.x, p2.y);
}
```

> ✅ `Point<T>` works for both `i32` and `f64`.

---

## ⚙️ Generic Struct with Multiple Types

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let mixed = Point { x: 5, y: 4.0 };

    println!("Mixed point: ({}, {})", mixed.x, mixed.y);
}
```

> ✅ You can use **multiple generic parameters** when needed.

---

## 📦 Generic Enum

```rust
enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let a = Option::Some(42);
    let b: Option<&str> = Option::None;
}
```

> ✅ Rust’s `Option` and `Result` are generic enums used everywhere in Rust.

---

## 🧪 Generic Methods

```rust
struct Container<T> {
    item: T,
}

impl<T> Container<T> {
    fn new(item: T) -> Self {
        Container { item }
    }

    fn get(&self) -> &T {
        &self.item
    }
}
```

> ✅ `impl<T>` allows defining methods for any type `T`.

---

## 📏 Trait Bounds (Constraints)

Sometimes, you want to **restrict what types** can be used with a generic. For that, use **trait bounds**.

```rust
fn print_debug<T: std::fmt::Debug>(item: T) {
    println!("{:?}", item);
}
```

> ✅ This function works only for types that implement the `Debug` trait.

---

## 💡 Common Traits for Bounds

| Trait       | Use Case                              |
| ----------- | ------------------------------------- |
| `Copy`      | Type can be copied instead of moved   |
| `Clone`     | Allows explicit `.clone()`            |
| `Debug`     | Allows using `{:?}` in `println!`     |
| `PartialEq` | Enables `==` and `!=` comparisons     |
| `Display`   | Enables `{}` formatting in `println!` |

---

## 🧠 `where` Clause

For **cleaner code** when bounds get long:

```rust
fn compare<T, U>(a: T, b: U)
where
    T: std::fmt::Debug,
    U: std::fmt::Display,
{
    println!("{:?} and {}", a, b);
}
```

---

## ✅ Summary

| Concept           | Description                                            |
| ----------------- | ------------------------------------------------------ |
| `T`, `U`, `V`     | Generic type parameters                                |
| Trait bounds      | Restrict generics to types implementing certain traits |
| Generic functions | One function for many types                            |
| Generic structs   | Define reusable data structures                        |
| `impl<T>`         | Implement methods for generic types                    |
| `where` clause    | Cleaner way to write trait bounds                      |

---

## 🚀 Benefits of Using Generics

✅ Avoid code duplication
✅ Improve flexibility and code reuse
✅ Safer than dynamic typing—**checked at compile time**
✅ Backbone of Rust standard library (like `Option<T>` and `Vec<T>`)

---

# 🧬 Traits in Rust

In Rust, a **trait** is a way to define shared behavior — similar to interfaces in other languages like Java or TypeScript.

Traits allow you to specify methods that types must implement. They are a cornerstone of Rust’s powerful type system.

---

## 🧪 Defining a Trait

```rust
trait Describe {
    fn describe(&self) -> String;
}
````

This trait defines a `describe` method that returns a `String`.

---

## 👷 Implementing a Trait

```rust
struct Book {
    title: String,
    author: String,
}

impl Describe for Book {
    fn describe(&self) -> String {
        format!("'{}' by {}", self.title, self.author)
    }
}
```

✅ Here, `Book` implements the `Describe` trait by defining the `describe` method.

---

## 🚀 Using Traits

```rust
fn print_description(item: &impl Describe) {
    println!("{}", item.describe());
}
```

You can accept any type that implements `Describe` using the `impl Trait` syntax.

Alternatively:

```rust
fn print_description<T: Describe>(item: &T) {
    println!("{}", item.describe());
}
```

Both versions work similarly — the second gives you more flexibility with generics.

---

## ✅ Example: Multiple Types Implementing Same Trait

```rust
struct Movie {
    name: String,
    director: String,
}

impl Describe for Movie {
    fn describe(&self) -> String {
        format!("'{}' directed by {}", self.name, self.director)
    }
}

fn main() {
    let book = Book {
        title: String::from("The Rust Book"),
        author: String::from("Rustaceans"),
    };

    let movie = Movie {
        name: String::from("Ferris Learns Rust"),
        director: String::from("Mozilla"),
    };

    print_description(&book);
    print_description(&movie);
}
```

---

## ➕ Default Implementations

You can provide a **default implementation** in the trait definition:

```rust
trait Describe {
    fn describe(&self) -> String {
        String::from("Description not available.")
    }
}
```

Now, types can either override `describe()` or use the default version.

---

## 🔗 Trait Bounds in Generics

```rust
fn compare_and_print<T: Describe>(item1: &T, item2: &T) {
    println!("Item 1: {}", item1.describe());
    println!("Item 2: {}", item2.describe());
}
```

Use trait bounds to ensure generic types implement certain behaviors.

---

## 🔁 Traits and Polymorphism

Traits enable **static dispatch** (compile-time polymorphism) and **dynamic dispatch** with `Box<dyn Trait>`:

```rust
fn print_boxed(item: Box<dyn Describe>) {
    println!("{}", item.describe());
}
```

Use `dyn Trait` when the concrete type is unknown at compile time but implements the trait.

---

## 🧠 Summary

| Concept                | Description                            |
| ---------------------- | -------------------------------------- |
| `trait TraitName`      | Define shared behavior                 |
| `impl Trait for Type`  | Implement behavior for a type          |
| `impl Trait` in params | Accept types that implement a trait    |
| `T: Trait`             | Trait bounds in generic functions      |
| `dyn Trait`            | Dynamic dispatch (trait objects)       |
| Default methods        | Provide fallback behavior in the trait |

---

## ✅ Best Practices

* Use traits to define abstract behavior shared across multiple types.
* Prefer `impl Trait` for simpler APIs.
* Use `dyn Trait` only when necessary (at runtime, with trade-offs in performance).
* Combine with generics for flexible and reusable code.

---

📚 Traits are at the heart of Rust’s **abstraction without compromise** philosophy. Mastering them opens up a powerful way to write expressive, modular, and reusable code.

---

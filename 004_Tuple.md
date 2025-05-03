#### 🧩 What is a Tuple?

A **tuple** in Rust is a **fixed-size** collection of values that can be of **different types**. Tuples are useful when you want to group related values together.

---

#### ✅ Syntax

```rust
let my_tuple = (value1, value2, value3);
```

Each value inside the tuple can be of a different type.

---

#### 🔍 Example 1: Basic Tuple

```rust
fn main() {
    let person: (&str, i32) = ("Alice", 30);

    println!("Name: {}, Age: {}", person.0, person.1);
}
```

**Output:**

```
Name: Alice, Age: 30
```

> Access tuple elements using dot notation: `.0`, `.1`, etc.

---

#### 🔍 Example 2: Destructuring a Tuple

```rust
fn main() {
    let coordinates = (4, 5);

    let (x, y) = coordinates;

    println!("X: {}, Y: {}", x, y);
}
```

---

#### 🔍 Example 3: Returning Tuples from Functions

```rust
fn get_name_and_age() -> (&'static str, u8) {
    ("Bob", 25)
}

fn main() {
    let (name, age) = get_name_and_age();
    println!("{} is {} years old.", name, age);
}
```

---

#### 🔍 Example 4: Nested Tuples

```rust
fn main() {
    let nested = ((1, 2), (3, 4));
    println!("First: {}, Second: {}", nested.0.0, nested.1.1);
}
```

---

#### 📝 When to Use Tuples?

* Group multiple values without creating a custom struct.
* Return multiple values from a function.
* Use with pattern matching and destructuring.

---

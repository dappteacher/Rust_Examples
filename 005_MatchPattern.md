#### 🎯 What is `match` in Rust?

`match` is a powerful **control flow** operator that allows you to compare a value against a series of patterns and execute code based on which pattern matches.

Think of it like a more powerful `switch` statement (from other languages) with pattern matching support.

---

#### ✅ Basic Syntax

```rust
match value {
    pattern1 => { code },
    pattern2 => { code },
    _ => { default_code }, // `_` is the catch-all pattern
}
```

---

#### 🔍 Example 1: Matching Numbers

```rust
fn main() {
    let number = 3;

    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Something else"),
    }
}
```

---

#### 🔍 Example 2: Matching with Enums

```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let dir = Direction::Left;

    match dir {
        Direction::Up => println!("Moving Up"),
        Direction::Down => println!("Moving Down"),
        Direction::Left => println!("Moving Left"),
        Direction::Right => println!("Moving Right"),
    }
}
```

---

#### 🔍 Example 3: Matching Ranges and Multiple Values

```rust
fn main() {
    let age = 17;

    match age {
        0..=12 => println!("Child"),
        13..=19 => println!("Teenager"),
        20 | 30 | 40 => println!("Nice round age!"),
        _ => println!("Adult"),
    }
}
```

---

#### 🔍 Example 4: Destructuring with Match

```rust
fn main() {
    let point = (3, 5);

    match point {
        (0, y) => println!("On Y axis at {}", y),
        (x, 0) => println!("On X axis at {}", x),
        (x, y) => println!("At point ({}, {})", x, y),
    }
}
```

---

#### 📝 Tips for Using `match`

* Always cover **all possible cases** (or use `_`).
* Works great with `Option`, `Result`, and custom `enum`s.
* Can destructure complex data types like tuples, structs, and enums.

---

#### 💡 Bonus: Match with `Option<T>`

```rust
fn main() {
    let name: Option<&str> = Some("Alice");

    match name {
        Some(n) => println!("Hello, {}!", n),
        None => println!("No name provided"),
    }
}
```

---

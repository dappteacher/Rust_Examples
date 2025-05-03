# 🧱 Structs in Rust

In Rust, **structs** (short for "structures") are used to **group related data** together into a single custom data type.

Think of a struct as a **blueprint for objects**, just like in other languages.

---

## 🧠 Why Use Structs?

Structs help you:

* Model real-world things (like a `User`, `Car`, or `Book`)
* Organize data clearly and safely
* Enable **methods** (functions associated with that data)

---

## 🔤 Defining a Struct

```rust
struct Person {
    name: String,
    age: u8,
}
```

This defines a struct `Person` with two fields: `name` and `age`.

---

## 🛠️ Creating and Using Structs

```rust
fn main() {
    let user = Person {
        name: String::from("Alice"),
        age: 30,
    };

    println!("Name: {}, Age: {}", user.name, user.age);
}
```

> ✅ You access fields using `.` notation, like `user.name`.

---

## 🧱 Structs Are Immutable by Default

```rust
let person = Person {
    name: String::from("Bob"),
    age: 45,
};

// person.age = 50; // ❌ ERROR: cannot assign to immutable field
```

To mutate, declare it as `mut`:

```rust
let mut person = Person {
    name: String::from("Bob"),
    age: 45,
};

person.age = 50; // ✅ OK
```

---

## 🧩 Struct Update Syntax

Reuse fields from an existing instance:

```rust
let person1 = Person {
    name: String::from("Charlie"),
    age: 28,
};

let person2 = Person {
    name: String::from("Dave"),
    ..person1
};
```

> ✅ `..person1` copies remaining fields (like `age`) from `person1`.

---

## 🧠 Tuple Structs

Tuple structs are **structs without named fields**, but with types.

```rust
struct Color(u8, u8, u8);

fn main() {
    let red = Color(255, 0, 0);
    println!("Red: {}, {}, {}", red.0, red.1, red.2);
}
```

---

## 🧰 Unit-like Structs

Structs without fields, often used for **markers or traits**.

```rust
struct Marker;

fn main() {
    let _m = Marker;
}
```

---

## ⚙️ Struct with Methods (`impl` Block)

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}
```

```rust
fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };

    println!("Area: {}", rect1.area());
    println!("Can hold rect2? {}", rect1.can_hold(&rect2));
}
```

> ✅ `&self` is shorthand for passing a reference to the instance.

---

## 📦 With Associated Functions

You can define constructors using `Self`:

```rust
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
```

---

## ✅ Summary

| Concept          | Description                                            |
| ---------------- | ------------------------------------------------------ |
| `struct`         | Defines a new custom type with named fields            |
| Tuple struct     | A struct with unnamed fields                           |
| Unit-like struct | Empty struct (used for markers)                        |
| `impl` block     | Adds methods or functions to a struct                  |
| `&self`          | Reference to the instance (like `this` in other langs) |
| `Self`           | Refers to the struct type itself                       |

---

## 🚀 Use Cases for Structs

✅ Group related data together
✅ Build real-world models (users, shapes, transactions, etc.)
✅ Add behavior to data via methods
✅ Enable clean, structured, readable code

---

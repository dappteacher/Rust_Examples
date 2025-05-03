# 🔒 Closures in Rust

Closures in Rust are **anonymous functions** you can store in variables, pass as arguments, and return from other functions. They’re similar to **lambdas** or **function literals** in other languages.

Closures are powerful because they can **capture variables from the environment**, giving them flexible and concise power for many tasks.

---

## 🧠 Basic Syntax

```rust
let closure_name = |parameter1, parameter2| {
    // code block
    return_value
};
```

Or in a shorter form:

```rust
let add = |a, b| a + b;
```

---

## 📌 Example 1: Simple Closure

```rust
fn main() {
    let greet = || println!("Hello from a closure!");
    greet(); // Call the closure
}
```

> ✅ A closure without parameters that simply prints a message.

---

## 📌 Example 2: Closure with Parameters

```rust
fn main() {
    let square = |x: i32| x * x;

    println!("5 squared is: {}", square(5)); // Output: 25
}
```

> ✅ A closure that takes an integer and returns its square.

---

## 🔍 Closures vs Functions

| Feature                 | Closure          | Function                      |
| ----------------------- | ---------------- | ----------------------------- |
| Can capture environment | ✅ Yes            | ❌ No                          |
| Syntax                  | Short and inline | Full `fn` definition required |
| Types                   | Often inferred   | Must be declared              |

---

## 📦 Capturing Environment

Closures can use variables **from the outer scope**:

```rust
fn main() {
    let name = String::from("Alice");

    let say_hello = || println!("Hello, {}", name);

    say_hello(); // ✅ uses `name` from outside
}
```

> ✅ Closures automatically capture what they need—by reference, mutable reference, or by value.

---

## ✨ Type Inference

Rust can often **infer types** in closures:

```rust
let multiply = |a, b| a * b;
```

You don’t always have to annotate types unless it’s ambiguous or required by context.

---

## ⚠️ Mutable Closures

To **mutate captured variables**, the closure must be declared `mut`, and variables captured must be `mut` too:

```rust
fn main() {
    let mut count = 0;

    let mut inc = || {
        count += 1;
        println!("Count: {}", count);
    };

    inc(); // Count: 1
    inc(); // Count: 2
}
```

> ⚠️ Notice how both `count` and the closure itself are marked `mut`.

---

## 🔁 Closures as Arguments

Closures can be passed into functions using `Fn`, `FnMut`, or `FnOnce` traits.

```rust
fn apply<F: Fn(i32)>(f: F, x: i32) {
    f(x);
}

fn main() {
    let print = |n| println!("Number: {}", n);
    apply(print, 7); // Output: Number: 7
}
```

---

## 🧪 Returning Closures from Functions

```rust
fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

fn main() {
    let add_five = make_adder(5);
    println!("5 + 3 = {}", add_five(3)); // Output: 8
}
```

> ✅ `move` is used to take ownership of captured values when returning closures.

---

## ✅ Summary Table

| Concept                 | Description                                          |
| ----------------------- | ---------------------------------------------------- |
| Closure                 | An anonymous function that can capture variables     |
| Type Inference          | Rust often infers closure parameter and return types |
| `move` keyword          | Forces closure to take ownership of captured values  |
| `Fn`, `FnMut`, `FnOnce` | Traits defining how closures capture and use values  |

---

## 💡 When to Use Closures

✅ Great for:

* Small operations passed to functions (`map`, `filter`, etc.)
* Capturing context (environment variables)
* Functional-style coding
* Passing logic into functions dynamically

---

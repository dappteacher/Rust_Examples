# 🔐 Mutex in Rust

A `Mutex` (Mutual Exclusion) in Rust is used to **safely share data between threads** by ensuring only one thread can access the data at a time. It's part of Rust's powerful concurrency model that emphasizes safety.

---

## 📦 Where Does `Mutex` Come From?

To use `Mutex`, import it from the standard library:

```rust
use std::sync::Mutex;
````

You often use it together with threads:

```rust
use std::sync::{Mutex, Arc};
use std::thread;
```

---

## 🚧 Why Use a `Mutex`?

When multiple threads access shared data **at the same time**, data races can occur. A `Mutex` ensures:

* Only one thread accesses the data at a time.
* Data races are **prevented**.
* Access to the data is **synchronized**.

---

## 🔓 Basic Example

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num += 1;
    }

    println!("Result: {:?}", m);
}
```

### 🔍 Explanation:

* `Mutex::new(5)` wraps the value `5` in a mutex.
* `.lock().unwrap()` locks the mutex and returns a `MutexGuard`, allowing access to the inner value.
* After the block ends, the lock is **automatically released**.

---

## 🧵 Sharing Mutex Across Threads

```rust
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

### 🧠 What’s New Here?

| Concept           | Description                                                                          |
| ----------------- | ------------------------------------------------------------------------------------ |
| `Arc<T>`          | Stands for "Atomic Reference Counted", used to share ownership safely across threads |
| `thread::spawn`   | Creates a new thread                                                                 |
| `lock().unwrap()` | Locks the mutex, panics on failure                                                   |

---

## ⚠️ Common Errors

* ❌ Trying to access `Mutex<T>` without locking.
* ❌ Forgetting to wrap the `Mutex` in `Arc` when using with threads.
* ❌ Keeping the lock too long—can cause **deadlocks**.

---

## 🔄 Using `try_lock()` Instead

```rust
if let Ok(mut data) = my_mutex.try_lock() {
    *data += 1;
} else {
    println!("Could not acquire lock!");
}
```

This method avoids blocking and only locks if available.

---

## 📦 Using Mutex in Structs

```rust
use std::sync::Mutex;

struct SharedState {
    data: Mutex<i32>,
}

fn main() {
    let state = SharedState {
        data: Mutex::new(0),
    };

    {
        let mut val = state.data.lock().unwrap();
        *val += 10;
    }

    println!("Updated: {}", *state.data.lock().unwrap());
}
```

---

## 🔐 When to Use Mutex

| Situation                          | Use Mutex?          |
| ---------------------------------- | ------------------- |
| Multiple threads write shared data | ✅ Yes               |
| Data needs interior mutability     | ✅ Yes               |
| Single-threaded program            | ❌ No                |
| You need atomic counters           | ❌ Use `AtomicUsize` |

---

## 📚 Further Reading

* 📘 [Rust Book - Shared-State Concurrency](https://doc.rust-lang.org/book/ch16-03-shared-state.html)
* 📖 [std::sync::Mutex docs](https://doc.rust-lang.org/std/sync/struct.Mutex.html)
* 🧠 [Rust Arc and Mutex Tutorial](https://doc.rust-lang.org/std/sync/struct.Arc.html)

---

## ✅ Summary

| Concept         | Purpose                                  |
| --------------- | ---------------------------------------- |
| `Mutex<T>`      | Ensures safe, exclusive access to data   |
| `lock()`        | Acquires the lock and returns a guard    |
| `Arc<Mutex<T>>` | Share locked data between threads        |
| `try_lock()`    | Non-blocking attempt to acquire the lock |

---

🚀 `Mutex` is essential when you need **shared mutability in multithreaded environments**. It brings safety and control to otherwise risky concurrent access. Master it to write safe concurrent Rust programs!

---

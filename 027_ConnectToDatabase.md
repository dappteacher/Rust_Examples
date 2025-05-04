# 🧩 Connecting Rust (Actix Web) to PostgreSQL

Combining **Actix Web** with **PostgreSQL** is a powerful and popular stack for building fast, scalable web APIs in Rust. In this tutorial, you'll create a **minimal Actix Web application** that connects to a PostgreSQL database and fetches user data from a table.

---

## 🎯 What We'll Build

A simple API endpoint (`GET /`) that retrieves users from a `users` table in PostgreSQL and displays them.

---

## 🛠️ Step-by-Step Guide

---

### 1️⃣ Add Dependencies

Edit your `Cargo.toml` to include:

```toml
[dependencies]
actix-web = "4"
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres"] }
dotenv = "0.15"
````

**What each crate does:**

* `actix-web`: Web framework.
* `tokio`: Async runtime.
* `sqlx`: Async Postgres driver.
* `dotenv`: Loads `.env` config for DB credentials.

---

### 2️⃣ Configure Environment

Create a `.env` file at the root of your project:

```env
DATABASE_URL=postgres://postgres:111@localhost/University
```

> Replace with your actual database credentials.

---

### 3️⃣ Create the `users` Table

Run the following SQL in your PostgreSQL database:

```sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL
);

INSERT INTO users (name) VALUES ('Alice'), ('Bob');
```

---

### 4️⃣ Write the Rust Code

In `src/main.rs`:

```rust
use actix_web::{web, App, HttpServer, Responder};
use sqlx::postgres::PgPoolOptions;
use sqlx::FromRow;
use dotenv::dotenv;
use std::env;

#[derive(FromRow, Debug)]
struct User {
    id: i32,
    name: String,
}

async fn get_users(pool: web::Data<sqlx::PgPool>) -> impl Responder {
    let result = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(pool.get_ref())
        .await;

    match result {
        Ok(users) => users
            .into_iter()
            .map(|user| format!("{}: {}", user.id, user.name))
            .collect::<Vec<_>>()
            .join("\n"),
        Err(e) => format!("Database error: {}", e),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Could not connect to the database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(get_users))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

---

### 5️⃣ Run Your Server

```bash
cargo run
```

Then visit:
👉 [http://127.0.0.1:8080/](http://127.0.0.1:8080/)

Output:

```
1: Alice
2: Bob
```

---

## ✅ Summary

You’ve just built a simple Actix Web app that:

* Connects to a PostgreSQL database using `sqlx`.
* Loads configuration from `.env`.
* Retrieves data from a real SQL table.
* Displays the data through a web endpoint.

---

## 📚 References

* [Actix Web Documentation](https://actix.rs/docs/)
* [SQLx GitHub](https://github.com/launchbadge/sqlx)
* [dotenv crate](https://docs.rs/dotenv/latest/dotenv/)

---

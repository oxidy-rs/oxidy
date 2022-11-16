# oxidy

Fast & Minimum Web Framework for Rust.

Built on top of Tokio Tcp with Tokio Runtime.

This project is highly inspired by **express.js**, **koa.js** & **warp.rs**.

## Features

- Main Focus on Fast & Performance
- Very minimum LOC (Lines of code)
- No Unsafe Code
- Tokio Tcp
- Tokio Runtime
- Robust Routing
- Allow Middleware
- Easy to build your own middleware
- Allow Multi Threading
- Allow Concurrency
- Full Async/Await Support

## Install

This is a crate (Rust Module) available on
[crate.io](https://crates.io/crates/oxidy). Before install oxidy
[download & install Rust](https://www.rust-lang.org/).

## Quick Start

- Add **oxidy** & **tokio** to your dependency in **Cargo.toml** file

```
[dependencies]
oxidy = "<version>"
tokio = { version = "<version>", features = ["full"] }
```

- Paste this code below in your **src/main.rs** file

```rust
use oxidy::{Server, Context, Returns, route};

async fn route(mut c: Context) -> Returns {
    c.response.body = "Hello World".to_owned();
    (c, None)
}

#[tokio::main]
async fn main() {
    let mut app = Server::new();
    app.add(route!("get /", route));
    app.run("127.0.0.1:3000").await;
}
```

- **cargo run** to run the server in development or **cargo run --release** to
  run the server in release profile.

## Middleware

```rust
use std::time::Instant;
use oxidy::{Server, Context, Returns, route, middleware, tail};

async fn mid(mut c: Context) -> Returns {
    let start = Instant::now();
    println!("Middleware Function");
    c.response.body = "Middleware Function".to_owned();
    c.next = true;

    tail!{
        c,
        {
            println!("Tail Function");
            c.response.body = "Tail Function".to_owned();
            println!("Response Time: {:?}", Instant::now().duration_since(start));
            c
        }
    }
}

async fn route(mut c: Context) -> Returns {
    println!("Route Function");
    c.response.body = "Hello World".to_owned();
    (c, None)
}

#[tokio::main]
async fn main() {
    let mut app = Server::new();
    app.add(middleware!(mid));
    app.add(route!("get /", route));
    app.run("127.0.0.1:3000").await;
}
```

## Note

- There is no difference between route & middleware in oxidy. All are same &
  identical.
- Try to use oxidy `profile.release` configuration to get highly optimize build.
  [Cargo.toml](Cargo.toml)
- This project is still in **Alpha**.

# License

This project is licensed under the **MIT** | [View License](LICENSE)

# Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for this project by you, shall be licensed as **MIT**, without any additional
terms or conditions.

# oxidy

Super Fast & High Performance minimalist web framework for rust

```rust
use oxidy::structs::Context;
use oxidy::server::Server;
use oxidy::structs::Middleware;
use std::time::Instant;

fn mid(_: &mut Context) -> Middleware {
    let start = Instant::now();
    (
        true,
        Some(Box::new(move |_: &mut Context| {
            let end = Instant::now();
            println!("Response Time: {:?}", end - start);
        })),
    )
}

fn index(ctx: &mut Context) -> () {
    ctx.response.body = "Index Page".to_string();
}

fn main() {
    let mut app = Server::new();
    app.middleware(mid);
    app.get("/", index);
    app.listen("127.0.0.1:3000");
}
```

## Description

Super Fast & High Performance minimalist web framework for rust built on top of
rust standard library TcpListener & TcpStream.

This project is highly inspired by

- Nodejs Express
- Nodejs Koa

## Features

- Main Focus on Super Fast & High Performance.
- Very minimum LOC (Lines of code).
- No Unsafe Code.
- Robust Routing.
- Allow Middlewares.
- Easy to build your own middleware;
- Allow Multi Threading.

## Install

This is a crate (Rust Module) available on
[crate.io](https://crates.io/crates/oxidy). Before install
[download & install rust](https://www.rust-lang.org/).

## Quick Start

- Add **oxidy** to your dependency in **Cargo.toml** file

```
[dependencies]
oxidy = "<version>"
```

- Paste this code below in your **src/main.rs** file

- **cargo run** to run the server in development or **cargo run --release** to
  run the server in release profile.

```rust
use oxidy::structs::Context;
use oxidy::server::Server;

fn index(ctx: &mut Context) -> () {
    ctx.response.body = "Index Page".to_string();
}

fn main() {
    let mut app = Server::new();
    app.get("/", index);
    app.listen("127.0.0.1:3000");
}
```

> Tested On Rust Stable Version & Edition 2021

# License

GNU GPL v2.0

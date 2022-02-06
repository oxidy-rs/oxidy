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

- **cargo run** to run the server in development or **cargo run --release** to
  run the server in release profile.

## Benchmark

### Apache Bench:

oxidy | Req/Sec: 16.5K | Latency: 60MS

actix web | Req/Sec: 15.5K | Latency: 63MS

### Loadtest

oxidy | Req/Sec: 4.5K | Latency: 219MS

actix web | Req/Sec: 2.9K | Latency: 339MS

[Check Full Benchmark](https://github.com/oxidy-rs/oxidy/blob/master/benchmark)

Tested (2022-02-06) On Rust Stable Version & Edition 2021 With Release Flag

# License

GNU GPL v2.0

extern crate threadpool;
mod libs;

mod server;
mod structs;

use server::Server;
use structs::Context;

async fn p_hello() -> () {
    println!("Hello World");
}

async fn index(ctx: &mut Context) -> () {
    ctx.response.body = "Hello".to_string();
    p_hello().await;
}

fn main() {
    let mut app = Server::new();
    app.get("/", index);
    app.listen("0.0.0.0:3000");
}

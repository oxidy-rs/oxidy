use crate::structs::definition::Callback;
use crate::utils::handler::handler;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio::spawn;

#[derive(Default, Clone)]
pub struct Server {
    pub(crate) adds: Vec<(String, String, Vec<Arc<Callback>>)>,
}

impl Server {
    /// New Server Instence
    ///
    /// # Example
    ///
    /// ```
    /// use oxidy::Server;
    ///
    /// /* Single Server */
    /// let mut app = Server::new();
    ///
    /// /* Multiple Servers */
    /// let mut app1 = Server::new();
    /// let mut app2 = Server::new();
    /// let mut app3 = Server::new();
    /// ```
    pub fn new() -> Server {
        Default::default()
    }
    /// Add Routes / Middlewares
    ///
    /// # Example
    ///
    /// ```
    /// use oxidy::{Server, Context, Returns, route};
    ///
    /// async fn route(mut c: Context) -> Returns {
    ///     c.response.body = "Get Route Function".to_owned();
    ///     (c, None)
    /// }
    ///
    /// let mut app = Server::new();
    /// app.add(route!("get /", route));
    /// ```
    pub fn add(&mut self, args: (&str, &str, Vec<Arc<Callback>>)) {
        self.adds
            .push((args.0.to_owned(), args.1.to_owned(), args.2));
    }
    /// Run / Listen
    ///
    /// # Example
    ///
    /// ```
    /// use oxidy::Server;
    ///
    /// let mut app = Server::new();
    /// /* app.run("127.0.0.1:3000").await; */
    /// ```
    pub async fn run(&self, address: &str) {
        /*
         * Bind Listener
         */
        let listener: TcpListener = TcpListener::bind(address)
            .await
            .expect("[Error] Fail to bind TCP Listener");
        /*
         * Connection Loop
         */
        loop {
            let listener_accept: Result<(TcpStream, SocketAddr), Error> = listener.accept().await;

            if listener_accept.is_err() {
                continue;
            }

            let (stream, address) = listener_accept.expect("[Error] Fail to Accept Connection");

            spawn(handler(self.to_owned(), address, stream));
        }
    }
}

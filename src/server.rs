use crate::libs::cpus::cpus;
use crate::libs::default_context::default_context;
use crate::libs::listen::listen;
use crate::structs::{Context, MiddlewareCallback, RouteCallback};
use futures::Future;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

#[derive(Clone)]
pub struct Server {
    pub(crate) middlewares: Vec<MiddlewareCallback>,
    pub(crate) gets: Vec<(String, RouteCallback)>,
    // pub(crate) posts: Vec<(String, RouteCallback)>,
    // pub(crate) puts: Vec<(String, RouteCallback)>,
    // pub(crate) deletes: Vec<(String, RouteCallback)>,
    // pub(crate) patchs: Vec<(String, RouteCallback)>,
    // pub(crate) catchs: Option<RouteCallback>,
    pub(crate) allow_threads: usize,
}

impl Server {
    /* /// Middlewares
    ///
    /// # Example
    ///
    /// ```
    /// use oxidy::structs::Context;
    /// use oxidy::structs::Middleware;
    /// use oxidy::server::Server;
    /// use std::time::Instant;
    ///
    /// fn mid (_: &mut Context) -> Middleware {
    ///     println!("Middleware Function");
    ///     let start = Instant::now();
    ///     (
    ///         true,
    ///         Some(Box::new(move |_: &mut Context| {
    ///             let end = Instant::now();
    ///             println!("Response Time: {:?}", end.duration_since(start));
    ///         })),
    ///     )
    /// }
    ///
    /// let mut app = Server::new();
    /// let a = app.middleware(mid);
    /// assert_eq!((), a);
    /// ``` */
    pub fn middleware(&mut self, callback: MiddlewareCallback) -> () {
        self.middlewares.push(callback);
    }
    /* /// GET Route
    ///
    /// # Example
    ///
    /// ```
    /// use oxidy::structs::Context;
    /// use oxidy::server::Server;
    ///
    /// fn index (_: &mut Context) -> () {
    ///     println!("Index GET Function");
    /// }
    ///
    /// let mut app = Server::new();
    /// let a = app.get("/", index);
    /// assert_eq!((), a);
    /// ``` */
    pub fn get<F, Fcb>(&mut self, path: &str, callback: F) -> ()
    where
        F: Fn(&mut Context) -> Fcb + 'static + Send,
        Fcb: Future<Output = ()> + 'static + Send,
    {
        self.gets.push((
            path.to_string(),
            Arc::new(Mutex::new(Box::new(move |_| {
                Box::pin(callback(&mut default_context()))
            }))),
        ));
    }
    /* /// POST Route
    ///
    /// # Example
    ///
    /// ```
    /// use oxidy::structs::Context;
    /// use oxidy::server::Server;
    ///
    /// fn user (_: &mut Context) -> () {
    ///     println!("User POST Function");
    /// }
    ///
    /// let mut app = Server::new();
    /// let a = app.post("/", user);
    /// assert_eq!((), a);
    /// ``` */
    // pub fn post(&mut self, path: &str, callback: impl Future<Output = ()> + 'static + Send) -> () {
    //     self.posts
    //         .push((path.to_string(), Arc::new(Mutex::new(Box::pin(callback)))));
    // }
    /* /// PUT Route
    ///
    /// # Example
    ///
    /// ```
    /// use oxidy::structs::Context;
    /// use oxidy::server::Server;
    ///
    /// fn user (_: &mut Context) -> () {
    ///     println!("User PUT Function");
    /// }
    ///
    /// let mut app = Server::new();
    /// let a = app.put("/", user);
    /// assert_eq!((), a);
    /// ``` */
    // pub fn put(&mut self, path: &str, callback: impl Future<Output = ()> + 'static + Send) -> () {
    //     self.puts
    //         .push((path.to_string(), Arc::new(Mutex::new(Box::pin(callback)))));
    // }
    /* /// DELETE Route
    ///
    /// # Example
    ///
    /// ```
    /// use oxidy::structs::Context;
    /// use oxidy::server::Server;
    ///
    /// fn user (_: &mut Context) -> () {
    ///     println!("User DELETE Function");
    /// }
    ///
    /// let mut app = Server::new();
    /// let a = app.delete("/", user);
    /// assert_eq!((), a);
    /// ``` */
    // pub fn delete(
    //     &mut self,
    //     path: &str,
    //     callback: impl Future<Output = ()> + 'static + Send,
    // ) -> () {
    //     self.deletes
    //         .push((path.to_string(), Arc::new(Mutex::new(Box::pin(callback)))));
    // }
    /* /// PATCH Route
    ///
    /// # Example
    ///
    /// ```
    /// use oxidy::structs::Context;
    /// use oxidy::server::Server;
    ///
    /// fn user (_: &mut Context) -> () {
    ///     println!("User PATCH Function");
    /// }
    ///
    /// let mut app = Server::new();
    /// let a = app.patch("/", user);
    /// assert_eq!((), a);
    /// ``` */
    // pub fn patch(&mut self, path: &str, callback: impl Future<Output = ()> + 'static + Send) -> () {
    //     self.patchs
    //         .push((path.to_string(), Arc::new(Mutex::new(Box::pin(callback)))));
    // }
    /* /// CATCH Method
    ///
    /// Catch Function will call on any HTTP Error
    ///
    /// # Example
    ///
    /// ```
    /// use oxidy::structs::Context;
    /// use oxidy::server::Server;
    ///
    /// fn catch (_: &mut Context) -> () {
    ///     println!("CATCH Function");
    /// }
    ///
    /// let mut app = Server::new();
    /// let a = app.catch(catch);
    /// assert_eq!((), a);
    /// ``` */
    // pub fn catch(&mut self, callback: impl Future<Output = ()> + 'static + Send) -> () {
    //     self.catchs = Some(Arc::new(Mutex::new(Box::pin(callback))));
    // }
    /// Multi Threading
    ///
    /// Number of Threads
    /// Default is 0 (Zero) Number of total CPU CORE
    ///
    /// # Example
    ///
    /// ```
    /// use oxidy::server::Server;
    ///
    /// let mut app = Server::new();
    /// let a = app.threads(1);
    /// assert_eq!((), a);
    /// ```
    pub fn threads(&mut self, allow: usize) -> () {
        self.allow_threads = allow;
    }
    /* /// Listen
    ///
    /// # Example
    ///
    /// ```
    /// use oxidy::server::Server;
    ///
    /// let mut app = Server::new();
    /// let a = app.listen("127.0.0.1:3000");
    /// assert_eq!((), a);
    /// ``` */
    pub fn listen(&self, address: &'static str) -> () {
        /*
         * Bind Server
         */
        let listener: TcpListener = TcpListener::bind(address).unwrap();
        /*
         * Thread Pool Size
         */
        let mut size: usize = self.allow_threads;
        if size < 1 {
            size = cpus();
        }
        /*
         * Thread Pool Create
         */
        let pool_listener: ThreadPool = ThreadPool::new(size);
        /*
         * Fork Listener
         */
        (0..size).for_each(|_| {
            let listener_cp: TcpListener = listener.try_clone().unwrap();
            let server_cp: Server = self.clone();
            pool_listener.execute(move || listen(listener_cp, server_cp));
        });

        drop(size);

        pool_listener.join();
    }
}
/* /// New Server Instence
///
/// # Example
///
/// ```
/// use oxidy::server::Server;
/// use oxidy::structs::Context;
///
/// fn index (_: &mut Context) -> () {
///     println!("Index GET Function");
/// }
///
/// fn user (_: &mut Context) -> () {
///     println!("User POST Function");
/// }
///
/// let mut app = Server::new();
/// app.get("/", index);
/// app.post("/", user);
/// ``` */
impl Server {
    /* /// New Server Instence
    ///
    /// # Example
    ///
    /// ```
    /// use oxidy::server::Server;
    /// use oxidy::structs::Context;
    ///
    /// fn index (_: &mut Context) -> () {
    ///     println!("Index GET Function");
    /// }
    ///
    /// fn user (_: &mut Context) -> () {
    ///     println!("User POST Function");
    /// }
    ///
    /// let mut app = Server::new();
    /// app.get("/", index);
    /// app.post("/", user);
    /// ``` */
    pub fn new() -> Server {
        Server {
            middlewares: Vec::new(),
            gets: Vec::new(),
            // posts: Vec::new(),
            // puts: Vec::new(),
            // deletes: Vec::new(),
            // patchs: Vec::new(),
            // catchs: None,
            allow_threads: 0,
        }
    }
}

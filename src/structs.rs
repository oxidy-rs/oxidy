use std::collections::HashMap;

/*
 * Request
 */
#[derive(Clone, Debug)]
pub struct Request {
    /// Get Request Haders
    ///
    /// # Example
    ///
    /// ```
    /// use oxidy::structs::Context;
    /// use oxidy::server::Server;
    ///
    /// fn index (ctx: &mut Context) {
    ///     let path = ctx.request.header.get("path").unwrap();
    ///     println!("Path: {}", path);
    /// }
    ///
    /// let mut app = Server::new();
    /// let a = app.get("/", index);
    /// assert_eq!((), a);
    /// ```
    pub header: HashMap<String, String>,
    /// Get Request Parameters
    ///
    /// # Example
    ///
    /// ```
    /// use oxidy::structs::Context;
    /// use oxidy::server::Server;
    ///
    /// fn user (ctx: &mut Context) {
    ///     let user = ctx.request.params.get("user").unwrap();
    ///     println!("User is: {}", user);
    /// }
    ///
    /// let mut app = Server::new();
    /// let a = app.get("/:user", user);
    /// assert_eq!((), a);
    /// ```
    pub params: HashMap<String, String>,
}

impl Request {
    /// Get Request Query Strings
    ///
    /// # Example
    ///
    /// ```
    /// use oxidy::structs::Context;
    /// use oxidy::server::Server;
    ///
    /// fn user (ctx: &mut Context) {
    ///     let user: String = ctx.request.query().get("user").unwrap().to_string();
    ///     println!("User is: {}", user);
    /// }
    ///
    /// let mut app = Server::new();
    /// let a = app.get("/user?user=username", user);
    /// assert_eq!((), a);
    /// ```
    pub fn query(&self) -> HashMap<String, String> {
        let mut query_str: HashMap<String, String> = HashMap::new();

        let query_hstr: String = self.header.get("query").unwrap().to_string();

        if query_hstr.is_empty() {
            return query_str;
        }

        let query_split: Vec<String> = query_hstr.split('&').map(|s| s.to_string()).collect();

        query_split.iter().for_each(|q| {
            let mut kv: Vec<String> = q.split('=').map(|s| s.to_string()).collect();

            if kv.get(0).is_none() || kv[0].is_empty() {
                return;
            }

            let k: String = kv[0].clone().to_lowercase();
            let mut v: String = "".to_string();

            if kv.get(1).is_some() {
                v = kv[1].clone();
                if kv.len() > 2 {
                    kv.remove(0);
                    v = kv.join("=");
                }
            }

            query_str.insert(k, v);
        });

        query_str
    }
}
/*
 * Response
 */
#[derive(Clone, Debug)]
pub struct Response {
    /// Set, Change & Remove Response Headers
    ///
    /// # Example
    ///
    /// ```
    /// use oxidy::structs::Context;
    /// use oxidy::server::Server;
    ///
    /// fn index (ctx: &mut Context) {
    ///     ctx.response.header.insert("status".to_string(), "200".to_string());
    ///     ctx.response.body = "<h1>Hello World</h1>".to_string();
    /// }
    ///
    /// let mut app = Server::new();
    /// let a = app.get("/", index);
    /// assert_eq!((), a);
    /// ```
    pub header: HashMap<String, String>,
    /// Get & Set Response Body
    ///
    /// # Example
    ///
    /// ```
    /// use oxidy::structs::Context;
    /// use oxidy::server::Server;
    ///
    /// fn index (ctx: &mut Context) {
    ///     /* To Get Body */
    ///     let body = &ctx.response.body;
    ///     println!("{}", body);
    ///
    ///     /* To Set Body */
    ///     ctx.response.body = "<h1>Hello World</h1>".to_string();
    /// }
    ///
    /// let mut app = Server::new();
    /// let a = app.get("/", index);
    /// assert_eq!((), a);
    /// ```
    pub body: String,
}
/*
 * Context
 */
#[derive(Clone, Debug)]
pub struct Context {
    pub request: Request,
    pub response: Response,
    /// State of Key Value pair to transfer data between Middlewares or Routes
    ///
    /// # Example
    ///
    /// ```
    /// use oxidy::structs::Context;
    /// use oxidy::structs::Middleware;
    /// use oxidy::server::Server;
    ///
    /// fn mid (ctx: &mut Context) -> Middleware {
    ///     ctx.state.insert("user".to_string(), "username".to_string());
    ///     (true, None)
    /// }
    ///
    /// fn index (ctx: &mut Context) {
    ///     ctx.response.body = ctx.state.get("user").unwrap().to_string();
    /// }
    ///
    /// let mut app = Server::new();
    /// app.middleware(mid);
    /// let a = app.get("/", index);
    /// assert_eq!((), a);
    /// ```
    pub state: HashMap<String, String>,
}
/*
 * Middleware Callback Return Type
 */
pub type Middleware = (bool, Option<Box<dyn Fn(&mut Context)>>);
pub(crate) type MiddlewareCallback = fn(&mut Context) -> Middleware;
/*
 * Route Callback Return Type
 */
pub(crate) type RouteCallback = fn(&mut Context);

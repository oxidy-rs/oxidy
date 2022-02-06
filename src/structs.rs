use std::collections::HashMap;

/*
 * Request
 */
pub struct Request {
    /// Get Request Haders
    ///
    /// # Example
    ///
    /// ```
    /// use oxidy::structs::Context;
    /// use oxidy::server::Server;
    ///
    /// fn index (ctx: &mut Context) -> () {
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
    /// fn user (ctx: &mut Context) -> () {
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
    /// fn user (ctx: &mut Context) -> () {
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
        if query_hstr.len() > 0 {
            let query_split: Vec<String> = query_hstr.split("&").map(|s| s.to_string()).collect();
            for q in query_split {
                let mut q_split: Vec<String> = q.split("=").map(|s| s.to_string()).collect();

                if q_split.get(0).is_none() || q_split[0].len() < 1 {
                    continue;
                }

                let k: String = q_split[0].clone().to_lowercase();
                let mut v: String = "".to_string();

                if q_split.get(1).is_some() {
                    v = q_split[1].clone();
                    if q_split.len() > 2 {
                        let _ = q_split.remove(0);
                        v = q_split.join("=").to_string();
                    }
                }

                query_str.insert(k, v);
            }
        }

        query_str
    }
}
/*
 * Response
 */
pub struct Response {
    /// Set, Change & Remove Response Headers
    ///
    /// # Example
    ///
    /// ```
    /// use oxidy::structs::Context;
    /// use oxidy::server::Server;
    ///
    /// fn index (ctx: &mut Context) -> () {
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
    /// fn index (ctx: &mut Context) -> () {
    ///     # To Get Body
    ///     let body = ctx.response.body;
    ///     println!("{}", body);
    ///
    ///     # To Set Body
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
pub struct Context {
    pub request: Request,
    pub response: Response,
    /// State of Key Value pair to transfer data between Middlewares or Routes
    ///
    /// # Example
    ///
    /// ```
    /// use oxidy::structs::Context;
    /// use oxidy::server::Server;
    ///
    /// fn mid (ctx: &mut Context) -> () {
    ///     ctx.state.insert("user".to_string(), "username".to_string());
    /// }
    ///
    /// fn index (ctx: &mut Context) -> () {
    ///     ctx.response.body = ctx.state.get("user").unwrap().to_string();
    /// }
    ///
    /// let mut app = Server::new();
    /// let a = app.get("/", index);
    /// assert_eq!((), a);
    /// ```
    pub state: HashMap<String, String>,
}
/*
 * Middleware Callback Return Type
 */
pub type Middleware = (bool, Option<Box<dyn Fn(&mut Context) -> ()>>);

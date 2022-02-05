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
    /// Get Request Query Strings
    ///
    /// # Example
    ///
    /// ```
    /// use oxidy::structs::Context;
    /// use oxidy::server::Server;
    ///
    /// fn user (ctx: &mut Context) -> () {
    ///     let user = ctx.request.query.get("user").unwrap();
    ///     println!("User is: {}", user);
    /// }
    ///
    /// let mut app = Server::new();
    /// let a = app.get("/user?user=username", user);
    /// assert_eq!((), a);
    /// ```
    pub query: HashMap<String, String>,
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

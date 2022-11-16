use crate::structs::request::Request;
use crate::structs::response::Response;
use crate::utils::del_vec::del_vec;
use crate::utils::get_vec::get_vec;
use crate::utils::set_vec::set_vec;

#[derive(Clone, Debug)]
pub struct Context {
    /// Next Function
    ///
    /// # Example
    ///
    /// ```
    /// use std::time::Instant;
    /// use oxidy::{Server, Context, Returns, middleware};
    ///
    /// async fn mid(mut c: Context) -> Returns {
    ///     let start: Instant = Instant::now();
    ///     c.response.body = "Middleware Function".to_owned();
    ///     c.next = true;
    ///     (c, None)
    /// }
    ///
    /// let mut app = Server::new();
    /// app.add(middleware!(mid));
    /// ```
    pub next: bool,
    pub(crate) state: Vec<(String, String)>,
    pub request: Request,
    pub response: Response,
}

impl Context {
    /// Get State
    ///
    /// State of Key Value pair to transfer data between Middlewares or Routes
    ///
    /// # Example
    ///
    /// ```
    /// use oxidy::{Server, Context, Returns, route, middleware};
    ///
    /// async fn mid(mut c: Context) -> Returns {
    ///     c.set_state("user", "John Doe").await;
    ///     (c, None)
    /// }
    ///
    /// async fn route(mut c: Context) -> Returns {
    ///     let user: Option<String> = c.get_state("user").await;
    ///     match user {
    ///         Some(u) => c.response.body = format!("Username: {}", u),
    ///         None => {
    ///             c.response.body = format!("Username not found");
    ///             c.response.status = 404;
    ///         },
    ///     }
    ///     (c, None)
    /// }
    ///
    /// let mut app = Server::new();
    /// app.add(middleware!(mid));
    /// app.add(route!("get /", route));
    /// ```
    pub async fn get_state(&self, key: &str) -> Option<String> {
        get_vec(&self.state, key.to_owned()).await
    }
    /// Set State
    ///
    /// State of Key Value pair to transfer data between Middlewares or Routes
    ///
    /// # Example
    ///
    /// ```
    /// use oxidy::{Server, Context, Returns, route, middleware};
    ///
    /// async fn mid(mut c: Context) -> Returns {
    ///     c.set_state("user", "John Doe").await;
    ///     (c, None)
    /// }
    ///
    /// async fn route(mut c: Context) -> Returns {
    ///     let user: Option<String> = c.get_state("user").await;
    ///     match user {
    ///         Some(u) => c.response.body = format!("Username: {}", u),
    ///         None => {
    ///             c.response.body = format!("Username not found");
    ///             c.response.status = 404;
    ///         },
    ///     }
    ///     (c, None)
    /// }
    ///
    /// let mut app = Server::new();
    /// app.add(middleware!(mid));
    /// app.add(route!("get /", route));
    /// ```
    pub async fn set_state(&mut self, key: &str, value: &str) {
        self.state = set_vec(&self.state, key.to_owned(), value.to_owned()).await;
    }
    /// Delete State
    ///
    /// State of Key Value pair to transfer data between Middlewares or Routes
    ///
    /// # Example
    ///
    /// ```
    /// use oxidy::{Server, Context, Returns, route, middleware};
    ///
    /// async fn mid(mut c: Context) -> Returns {
    ///     c.del_state("user").await;
    ///     (c, None)
    /// }
    ///
    /// async fn route(mut c: Context) -> Returns {
    ///     let user: Option<String> = c.get_state("user").await;
    ///     match user {
    ///         Some(u) => c.response.body = format!("Username: {}", u),
    ///         None => {
    ///             c.response.body = format!("Username not found");
    ///             c.response.status = 404;
    ///         },
    ///     }
    ///     (c, None)
    /// }
    ///
    /// let mut app = Server::new();
    /// app.add(middleware!(mid));
    /// app.add(route!("get /", route));
    /// ```
    pub async fn del_state(&mut self, key: &str) {
        self.state = del_vec(&self.state, key.to_owned()).await;
    }
}

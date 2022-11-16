use crate::utils::del_vec::del_vec;
use crate::utils::get_vec::get_vec;
use crate::utils::set_vec::set_vec;

use serde::Serialize;
use serde_json::Error;

#[derive(Clone, Debug)]
pub struct Response {
    pub(crate) header: Vec<(String, String)>,
    /// Get & Set Response Body
    ///
    /// # Example
    ///
    /// ```
    /// use oxidy::{Server, Context, Returns, route};
    ///
    /// async fn route(mut c: Context) -> Returns {
    ///     /* Get */
    ///     let body: String = c.response.body;
    ///     println!("{}", body);
    ///
    ///     /* Set */
    ///     c.response.body = "<h1>Hello World</h1>".to_owned();
    ///     (c, None)
    /// }
    ///
    /// let mut app = Server::new();
    /// app.add(route!("get /", route));
    /// ```
    pub body: String,
    /// Get & Set Response Status
    ///
    /// # Example
    ///
    /// ```
    /// use oxidy::{Server, Context, Returns, route};
    ///
    /// async fn route(mut c: Context) -> Returns {
    ///     /* Get */
    ///     let status: usize = c.response.status;
    ///     println!("{}", status);
    ///
    ///     /* Set */
    ///     c.response.status = 200;
    ///     (c, None)
    /// }
    ///
    /// let mut app = Server::new();
    /// app.add(route!("get /", route));
    /// ```
    pub status: usize,
    /// Get & Set Response Content Type
    ///
    /// # Example
    ///
    /// ```
    /// use oxidy::{Server, Context, Returns, route};
    ///
    /// async fn route(mut c: Context) -> Returns {
    ///     /* Get */
    ///     let content_type: String = c.response.content_type;
    ///     println!("{}", content_type);
    ///
    ///     /* Set */
    ///     c.response.content_type = "text/xml".to_owned();
    ///     (c, None)
    /// }
    ///
    /// let mut app = Server::new();
    /// app.add(route!("get /", route));
    /// ```
    pub content_type: String,
}

impl Response {
    /// Set JSON Response Body
    ///
    /// # Example
    ///
    /// ```
    /// use oxidy::{Server, Context, Returns, route};
    /// use serde::Serialize;
    ///
    /// #[derive(Serialize)]
    /// struct User {
    ///     name: String,
    ///     age: u32,
    /// }
    ///
    /// async fn route(mut c: Context) -> Returns {
    ///     c.response
    ///         .json(User {
    ///             name: "John Doe".to_owned(),
    ///             age: 10,
    ///         })
    ///         .await;
    ///
    ///     (c, None)
    /// }
    ///
    /// let mut app = Server::new();
    /// app.add(route!("get /", route));
    /// ```
    pub async fn json(&mut self, value: impl Serialize) {
        let value: Result<String, Error> = serde_json::to_string(&value);
        match value {
            Ok(s) => self.body = s,
            Err(e) => {
                self.body = "{}".to_owned();
                println!("[Error] Fail to serialize json data:\n{}", e);
            }
        }
        self.content_type = "application/json".to_owned();
    }
    /// Get Response Header
    ///
    /// # Example
    ///
    /// ```
    /// use oxidy::{Server, Context, Returns, route};
    ///
    /// async fn route(c: Context) -> Returns {
    ///     let auth: Option<String> = c.response.get_header("Authorization").await;
    ///     match auth {
    ///         Some(a) => println!("Auth is: {}", a),
    ///         None => println!("No auth found"),
    ///     }
    ///
    ///     (c, None)
    /// }
    ///
    /// let mut app = Server::new();
    /// app.add(route!("get /", route));
    /// ```
    pub async fn get_header(&self, key: &str) -> Option<String> {
        get_vec(&self.header, key.to_owned()).await
    }
    /// Set Response Header
    ///
    /// # Example
    ///
    /// ```
    /// use oxidy::{Server, Context, Returns, route};
    ///
    /// async fn route(mut c: Context) -> Returns {
    ///     c.response.set_header("Authorization", "Basic abcd1234").await;
    ///     (c, None)
    /// }
    ///
    /// let mut app = Server::new();
    /// app.add(route!("get /", route));
    /// ```
    pub async fn set_header(&mut self, key: &str, value: &str) {
        self.header = set_vec(&self.header, key.to_owned(), value.to_owned()).await;
    }
    /// Delete Response Header
    ///
    /// # Example
    ///
    /// ```
    /// use oxidy::{Server, Context, Returns, route};
    ///
    /// async fn route(mut c: Context) -> Returns {
    ///     c.response.del_header("Authorization").await;
    ///     (c, None)
    /// }
    ///
    /// let mut app = Server::new();
    /// app.add(route!("get /", route));
    /// ```
    pub async fn del_header(&mut self, key: &str) {
        self.header = del_vec(&self.header, key.to_owned()).await;
    }
}

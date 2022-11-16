use crate::utils::get_vec::get_vec;

#[derive(Clone, Debug)]
pub struct Request {
    /// Get Client Address/IP with Port
    ///
    /// # Example
    ///
    /// ```
    /// use oxidy::{Server, Context, Returns, route};
    ///
    /// async fn route(c: Context) -> Returns {
    ///     println!("Path: {}", c.request.address);
    ///     (c, None)
    /// }
    ///
    /// let mut app = Server::new();
    /// app.add(route!("get /", route));
    /// ```
    pub address: String,
    /// Get Request Header (RAW)
    ///
    /// # Example
    ///
    /// ```
    /// use oxidy::{Server, Context, Returns, route};
    ///
    /// async fn route(c: Context) -> Returns {
    ///     println!("Path: {}", c.request.header);
    ///     (c, None)
    /// }
    ///
    /// let mut app = Server::new();
    /// app.add(route!("get /", route));
    /// ```
    pub header: String,
    /*
     * Store / Cache
     */
    pub(crate) header_store: Vec<(String, String)>,
    pub(crate) param_store: Vec<(String, String)>,
    pub(crate) query_store: Vec<(String, String)>,
    pub method: String,
    pub url: String,
    pub path: String,
    pub query: String,
    pub http_version: f64,
}

impl Request {
    /// Get Request Header
    ///
    /// # Example
    ///
    /// ```
    /// use oxidy::{Server, Context, Returns, route};
    ///
    /// async fn route(mut c: Context) -> Returns {
    ///     let host: Option<String> = c.request.header("host").await;
    ///     match host {
    ///         Some(h) => println!("Host is: {}", h),
    ///         None => println!("Host not found"),
    ///     }
    ///     (c, None)
    /// }
    ///
    /// let mut app = Server::new();
    /// app.add(route!("get /", route));
    /// ```
    pub async fn header(&mut self, key: &str) -> Option<String> {
        if !self.header_store.is_empty() {
            return get_vec(&self.header_store, key.to_owned()).await;
        }

        let mut found_value: Option<String> = None;

        let mut headers: Vec<(String, String)> = Vec::new();

        let mut h: Vec<&str> = self.header.lines().collect();
        h.remove(0);

        h.into_iter().for_each(|ln: &str| {
            let mut ln_split: Vec<String> =
                ln.split_whitespace().map(|s: &str| s.to_owned()).collect();

            if ln_split.is_empty() {
                return;
            }
            /*
             * Key
             */
            let k: String = ln_split[0].trim().replace(':', "");
            /*
             * Filter Key
             */
            let k: Vec<String> = k.split_whitespace().map(|s: &str| s.to_owned()).collect();
            let k: String = k.join("");
            if k.is_empty() {
                return;
            }
            /*
             * Value
             */
            let mut v: String = String::new();
            ln_split.remove(0);

            if !ln_split.is_empty() {
                v = ln_split.join(" ");
            }

            if k == key {
                found_value = Some(v.clone());
            }

            headers.push((k, v));
        });

        self.header_store = headers;
        found_value
    }
    /// Get Request Parameter
    ///
    /// # Example
    ///
    /// ```
    /// use oxidy::{Server, Context, Returns, route};
    ///
    /// async fn route(mut c: Context) -> Returns {
    ///     let user: String = c.request.param("user").await;
    ///     c.response.body = format!("Username: {}", user);
    ///     (c, None)
    /// }
    ///
    /// let mut app = Server::new();
    /// app.add(route!("get /:user", route));
    /// ```
    pub async fn param(&self, key: &str) -> String {
        let v: Option<String> = get_vec(&self.param_store, key.to_owned()).await;
        match v {
            Some(x) => x,
            None => String::new(),
        }
    }
    /// Get Request Query
    ///
    /// # Example
    ///
    /// ```
    /// use oxidy::{Server, Context, Returns, route};
    ///
    /// async fn route(mut c: Context) -> Returns {
    ///     let user: Option<String> = c.request.query("user").await;
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
    ///
    /// /* Requested URL: /user?user=John */
    /// app.add(route!("get /", route));
    /// ```
    pub async fn query(&mut self, key: &str) -> Option<String> {
        if !self.query_store.is_empty() {
            return get_vec(&self.query_store, key.to_owned()).await;
        }

        let mut found_value: Option<String> = None;

        let query: String = self.query.clone();

        if query.is_empty() {
            return found_value;
        }

        let mut query_str: Vec<(String, String)> = Vec::new();
        let query_split: Vec<String> = query.split('&').map(|s: &str| s.to_owned()).collect();

        query_split.iter().for_each(|q: &String| {
            let mut kv: Vec<String> = q.split('=').map(|s: &str| s.to_owned()).collect();

            if kv.get(0).is_none() || kv[0].is_empty() {
                return;
            }

            let k: String = kv[0].to_owned();
            let mut v: String = String::new();
            kv.remove(0);

            if !kv.is_empty() {
                v = kv.join("=");
            }

            if k == key {
                found_value = Some(v.clone());
            }

            query_str.push((k, v));
        });

        self.query_store = query_str;
        found_value
    }
}

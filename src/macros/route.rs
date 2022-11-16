/// Route Macro
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
#[macro_export]
macro_rules! route {
    ($method_path:expr, $($func:tt),*) => {{
        use std::sync::Arc;
        use $crate::structs::definition::Callback;
        use $crate::structs::context::Context;

        let method_path_split: Vec<&str> = $method_path.split_whitespace().collect();
        /*
         * Get Method
         */
        let method: &str = match method_path_split.get(0) {
            Some(x) => {
                if x.is_empty() {
                    "*"
                } else {
                    x
                }
            }
            None => "*",
        };
        /*
         * Get Path
         */
        let path: &str = match method_path_split.get(1) {
            Some(x) => {
                if x.is_empty() {
                    "*"
                } else {
                    x
                }
            }
            None => "*",
        };
        /*
         * Function Vec
         */
        let mut funcs: Vec<Arc<Callback>> = Vec::new();
        $(
            funcs.push(Arc::new(Box::new(move |c: Context| Box::pin($func(c)))));
        )*

        (method, path, funcs)
    }};
}

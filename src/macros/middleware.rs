/// Middleware Macro
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
///     (c, None)
/// }
///
/// let mut app = Server::new();
/// app.add(middleware!(mid));
/// ```
#[macro_export]
macro_rules! middleware {
    ($func:tt) => {{
        use std::sync::Arc;
        use $crate::structs::context::Context;
        use $crate::structs::definition::Callback;
        /*
         * Function Vec
         */
        let mut funcs: Vec<Arc<Callback>> = Vec::new();
        funcs.push(Arc::new(Box::new(move |c: Context| Box::pin($func(c)))));

        ("*", "*", funcs)
    }};
}

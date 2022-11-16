/// Tail Macro
///
/// # Example
///
/// ```
/// use std::time::Instant;
/// use oxidy::{Server, Context, Returns, middleware, tail};
///
/// async fn mid(mut c: Context) -> Returns {
///     let start: Instant = Instant::now();
///     c.response.body = "Middleware Function".to_owned();
///
///     tail!{
///         c,
///         {
///             c.response.body = "Tail Function".to_owned();
///             let end: Instant = Instant::now();
///             println!("Response Time: {:?}", end.duration_since(start));
///             c
///         }
///     }
/// }
///
/// let mut app = Server::new();
/// app.add(middleware!(mid));
/// ```
#[macro_export]
macro_rules! tail {
    ($context:ident, $func:block) => {
        (
            $context,
            Some(Box::new(move |mut $context| Box::pin(async move { $func }))),
        )
    };
}

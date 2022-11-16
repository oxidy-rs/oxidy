use crate::structs::context::Context;
use futures::future::BoxFuture;

pub type Callback = Box<dyn Fn(Context) -> BoxFuture<'static, Returns> + Send + Sync>;

pub type Returns = (Context, Option<Tail>);

pub(crate) type Tail = Box<dyn Fn(Context) -> BoxFuture<'static, Context> + Send + Sync>;

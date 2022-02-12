use crate::structs::{Context, Request, Response};
use std::collections::HashMap;

/*
 * Get Default Context
 */
pub(crate) fn default_context() -> Context {
    Context {
        request: Request {
            header: HashMap::new(),
            params: HashMap::new(),
        },
        response: Response {
            header: HashMap::new(),
            body: "".to_string(),
        },
        state: HashMap::new(),
    }
}

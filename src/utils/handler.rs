use crate::server::Server;
use crate::structs::context::Context;
use crate::structs::definition::{Callback, Returns, Tail};
use crate::structs::request::Request;
use crate::structs::response::Response;
use crate::utils::find_callback::{find_callback, IsFind};
use crate::utils::get_header::get_header;
use crate::utils::parse_http_version::parse_http_version;
use crate::utils::parse_method::parse_method;
use crate::utils::parse_path::parse_path;
use crate::utils::response_payload::response_payload;
use crate::utils::response_payload_empty::response_payload_empty;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::join;
use tokio::net::TcpStream;

/*
 * Handler
 */
pub(crate) async fn handler(server: Server, address: SocketAddr, stream: TcpStream) {
    let (reader, writer) = stream.into_split();

    let header: String = get_header(reader).await;

    if header.is_empty() {
        response_payload_empty(writer).await;
        return;
    }
    /*
     * Incoming
     */
    let ((url, path, query), method, http_version) = join!(
        parse_path(header.clone()),
        parse_method(header.clone()),
        parse_http_version(header.clone())
    );
    /*
     * Context
     */
    let mut context: Context = Context {
        next: true,
        state: Vec::new(),
        request: Request {
            address: address.to_string(),
            header,
            header_store: Vec::new(),
            param_store: Vec::new(),
            query_store: Vec::new(),
            method: method.clone(),
            url,
            path: path.clone(),
            query,
            http_version,
        },
        response: Response {
            header: Vec::new(),
            body: String::new(),
            status: 200,
            content_type: "text/html".to_owned(),
        },
    };
    /*
     * Find & Callback
     */
    let mut tails: Vec<Tail> = Vec::new();

    let adds: Vec<(String, String, Vec<Arc<Callback>>)> = server.adds;

    for add in adds.iter() {
        if !context.next {
            break;
        }

        let method_cp: String = if add.0 == "*" {
            method.clone().to_lowercase()
        } else {
            add.0.to_lowercase()
        };

        let path_cp: String = if add.1 == "*" {
            path.clone().to_lowercase()
        } else {
            add.1.to_lowercase()
        };

        let callback_cp: Vec<Arc<Callback>> = add.2.to_owned();

        if method_cp != method.to_lowercase() {
            continue;
        }
        /*
         * Static
         */
        if path_cp == path.to_lowercase() {
            for callback in callback_cp.clone() {
                context.next = false;

                context.request.param_store = Vec::new();

                let callback_returns: Returns = (callback)(context).await;

                context = callback_returns.0;

                if let Some(x) = callback_returns.1 {
                    tails.push(x);
                }

                if !context.next {
                    break;
                }
            }

            continue;
        }
        /*
         * Dynamic
         */
        let find_callback: IsFind = find_callback(path.to_owned(), path_cp).await;

        if find_callback.find {
            for callback in callback_cp.clone() {
                context.next = false;

                context.request.param_store = find_callback.param.to_owned();

                let callback_returns: Returns = (callback)(context).await;

                context = callback_returns.0;

                if let Some(x) = callback_returns.1 {
                    tails.push(x);
                }

                if !context.next {
                    break;
                }
            }
        }
    }
    /*
     * Route Not Found
     */
    if context.next {
        /*
         * Default Status & Body
         */
        context.response.status = 404;
        context.response.body = "Not Found".to_owned();
    }
    /*
     * Tail
     */
    if !tails.is_empty() {
        for i in tails.iter().rev() {
            context = (i)(context).await;
            if !context.next {
                break;
            }
        }
    }

    response_payload(writer, context, http_version).await;
}

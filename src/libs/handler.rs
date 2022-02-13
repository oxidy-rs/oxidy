use crate::libs::found_callback::{found_callback, IsFound};
use crate::libs::http_status_string::http_status_string;
use crate::libs::parse::parse;
use crate::server::Server;
use crate::structs::{Context, MiddlewareCallback, Request, Response, RouteCallback};
use futures::future::join_all;
use std::borrow::Cow;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;

/*
 * Handler
 */
pub(crate) async fn handler(mut stream: TcpStream, server: Server) -> () {
    /*
     * Buffer
     */
    let mut buffer: [u8; 1024] = [0; 1024];
    stream.read(&mut buffer).unwrap();
    /*
     * Request Header
     */
    let header: Cow<str> = String::from_utf8_lossy(&buffer[..]);
    let mut header: HashMap<String, String> = parse(header.to_string()).await;
    /*
     * Client IP
     */
    let client_ip: String = stream.peer_addr().unwrap().to_string();
    let client_ip: String = client_ip.split(":").next().unwrap_or("").to_string();
    header.insert("ip".to_string(), client_ip);
    /*
     * Response Header Default
     */
    let mut response_header: HashMap<String, String> = HashMap::new();
    response_header.insert("status".to_string(), "200".to_string());
    response_header.insert("type".to_string(), "text/html".to_string());
    /*
     * Context
     */
    let mut context: Context = Context {
        request: Request {
            header: header.clone(),
            params: HashMap::new(),
        },
        response: Response {
            header: response_header,
            body: "".to_string(),
        },
        state: HashMap::new(),
    };
    /*
     * Incoming Path
     */
    let path: String = header.get("path").unwrap().to_string().to_lowercase();
    /*
     * Next Execution
     */
    let mut next_exec: bool = true;
    /*
     * Middlewares
     */
    let r: Vec<MiddlewareCallback> = server.middlewares;

    let mut middleware_ends: Vec<Box<dyn Fn(&mut Context) -> ()>> = Vec::new();

    for i in r {
        let (next, next_callback) = (i)(&mut context);

        if !next {
            next_exec = false;
            break;
        }

        if next_callback.is_some() {
            middleware_ends.push(next_callback.unwrap());
        }
    }

    if next_exec {
        /*
         * Routes
         */
        let r: Vec<(String, RouteCallback)> = match header.get("method").unwrap().as_str() {
            "GET" => server.gets,
            "POST" => server.posts,
            "PUT" => server.puts,
            "DELETE" => server.deletes,
            "PATCH" => server.patchs,
            _ => Vec::new(),
        };

        let mut is_found: bool = false;
        /*
         * Find & Call Route Callback
         */
        let found_callback_iter = r
            .iter()
            .map(|i| found_callback(path.clone(), i.0.to_lowercase()));

        let found_callback_iter: Vec<IsFound> = join_all(found_callback_iter).await;

        for (i, el) in r.iter().enumerate() {
            if found_callback_iter[i].found && found_callback_iter[i].path == el.0.to_lowercase() {
                context.request.params = found_callback_iter[i].params.clone();
                (el.1)(&mut context);
                is_found = true;
                break;
            }
        }

        drop(found_callback_iter);
        /*
         * Error
         */
        if !is_found {
            /*
             * Status
             */
            context
                .response
                .header
                .insert("status".to_string(), "404".to_string());
            /*
             * Check Catch Callback Exists
             */
            if server.catchs.is_some() {
                let cb = server.catchs.unwrap();
                (cb)(&mut context);
            } else {
                /*
                 * Default Body
                 */
                context.response.body = "404 Not Found".to_string();
            }
        }
        /*
         * Middlewares End
         */
        middleware_ends.iter().rev().for_each(|i| (i)(&mut context));
    }
    /*
     * Prepare Response Headers
     */
    let mut response_header: String = String::from("");
    context.response.header.iter().for_each(|(k, v)| {
        if k == "status" || k == "type" {
            return;
        }
        response_header.push_str(&format!("{}: {}\r\n", k, v));
    });
    /*
     * Prepare Response Payload
     */
    let status: String = context.response.header.get("status").unwrap().to_string();
    let status_str: String = http_status_string(status.clone());

    let response: String = format!(
        "HTTP/{0} {1} {2}\r\n{3}Context-Type: {4}\r\nContent-Length: {5}\r\n\r\n{6}",
        header.get("http-version").unwrap(),
        status,
        status_str,
        response_header,
        context.response.header.get("type").unwrap(),
        context.response.body.len(),
        context.response.body,
    );
    /*
     * Flush Payload
     */
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

use crate::libs::http_status_string::http_status_string;
use crate::libs::parse::parse;
use crate::server::MiddlewareCallback;
use crate::server::Server;
use crate::structs::{Context, Request, Response};
use std::borrow::Cow;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;

/*
 * Handler
 */
pub(crate) fn handler(mut stream: TcpStream, server: Server) -> () {
    /*
     * Buffer
     */
    let mut buffer: [u8; 1024] = [0; 1024];
    stream.read(&mut buffer).unwrap();
    /*
     * Request Header
     */
    let header: Cow<str> = String::from_utf8_lossy(&buffer[..]);
    let mut header: HashMap<String, String> = parse(header.to_string());
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
    let mut path_split: Vec<String> = path.clone().split("/").map(|s| s.to_string()).collect();
    path_split.remove(0);
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
        let mut r: Vec<(String, fn(&mut Context) -> ())> = Vec::new();

        if header.get("method").unwrap() == "GET" {
            r = server.gets;
        } else if header.get("method").unwrap() == "POST" {
            r = server.posts;
        } else if header.get("method").unwrap() == "PUT" {
            r = server.puts;
        } else if header.get("method").unwrap() == "DELETE" {
            r = server.deletes;
        } else if header.get("method").unwrap() == "PATCH" {
            r = server.patchs;
        }

        let mut is_found: bool = false;

        for i in r {
            /*
             * Current Path
             */
            let path_curr: String = i.0.to_lowercase();
            /*
             * Static Match
             */
            if path_curr == path {
                (i.1)(&mut context);
                is_found = true;
                break;
            }
            /*
             * Dynamic Match
             */
            let mut path_curr_split: Vec<String> = Vec::new();

            for p in path_curr.split("/") {
                let p = p.to_string();
                if p.len() > 0 {
                    path_curr_split.push(p);
                }
            }
            /*
             * Check Split Length
             */
            if path_curr_split.len() != path_split.len() {
                continue;
            }

            let mut prepare_path: String = String::from("");
            /*
             * Check Params
             */
            for j in 0..path_curr_split.len() {
                let path_elm: String = path_curr_split[j].clone();
                let path_char: String = path_elm.clone().chars().nth(0).unwrap().to_string();
                /*
                 * Dynamic Param
                 */
                if path_char == ":" {
                    prepare_path.push_str(&format!("/{}", path_elm));
                    context
                        .request
                        .params
                        .insert(path_elm.replace(":", ""), path_split[j].to_string());
                }
                /*
                 * Static Param
                 */
                else if path_elm == path_split[j] {
                    prepare_path.push_str(&format!("/{}", path_elm));
                }
            }
            /*
             * Match current path with prepare path
             */
            if path_curr == prepare_path {
                (i.1)(&mut context);
                is_found = true;
                break;
            }
        }
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
            let catch_route: bool = match server.catchs {
                None => false,
                _ => true,
            };

            if catch_route {
                (server.catchs).unwrap()(&mut context);
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
        for i in middleware_ends.iter().rev() {
            (i)(&mut context);
        }
    }
    /*
     * Prepare Response Headers
     */
    let mut response_header: String = String::from("");
    for (k, v) in &context.response.header {
        if k == "status" || k == "type" {
            continue;
        }
        response_header.push_str(&format!("{}: {}\r\n", k, v));
    }
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

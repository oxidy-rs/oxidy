use crate::libs::handler::handler;
use crate::server::Server;
use futures::executor::block_on;
use std::net::TcpListener;

/*
 * Listener to Fork
 */
pub(crate) fn listen(listener: TcpListener, server: Server) -> () {
    loop {
        let accept = listener.accept();
        match accept {
            Ok((stream, _)) => {
                let server_cp: Server = server.clone();
                block_on(handler(stream, server_cp));
            }
            Err(_) => (),
        };
    }
}

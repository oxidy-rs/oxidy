use crate::libs::handler::handler;
use crate::server::Server;
use futures::executor::block_on;
use std::net::TcpListener;

/*
 * Listener to Fork
 */
pub(crate) fn listen(listener: TcpListener, server: Server) {
    loop {
        if let Ok((stream, _)) = listener.accept() {
            let server_cp: Server = server.clone();
            block_on(handler(stream, server_cp));
        }
    }
}

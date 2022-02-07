use crate::libs::handler::handler;
use crate::server::Server;
use std::net::TcpListener;
use threadpool::ThreadPool;

/*
 * Listener to Fork
 */
pub(crate) fn fork(pool: ThreadPool, listener: TcpListener, server: Server) -> () {
    loop {
        let accept = listener.accept();
        match accept {
            Ok((stream, _)) => {
                let pool_cp: ThreadPool = pool.clone();
                let server_cp: Server = server.clone();
                pool_cp.execute(move || handler(stream, server_cp));
            }
            Err(_) => (),
        };
    }
}

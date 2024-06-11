use http::server::Server;
mod http;
fn main() {

    let addr = "127.0.0.1".to_owned();
    let port: u32 = 8083;

    let server = Server::new(addr, Some(port));
    
    server.run();


}


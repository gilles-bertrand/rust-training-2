mod handler;
mod server;
mod router;
use server::Server;

fn main() {
    //STart a server
    let server = Server::new("localhost:3000");
    server.run();
}

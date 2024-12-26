mod data;
mod data_manager;
mod server;

use server::Server;

fn main() {
    let server = Server::default();

    server.start();
}

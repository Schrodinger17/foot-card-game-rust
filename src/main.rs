mod calendar;
mod card;
mod data;
mod data_manager;
mod id;
mod matches;
mod player;
mod server;
mod shop;
mod user;

use server::Server;

fn main() {
    let mut server = Server::new(2025, 0);

    server.load();

    server.start();
}

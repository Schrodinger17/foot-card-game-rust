mod calendar;
mod card;
mod data;
mod data_manager;
mod id_manager;
mod matches;
mod player;
mod server;
mod shop;
mod user;

use server::Server;

fn main() {
    let mut server = Server::default();

    //server.load_data();

    server.start();
}

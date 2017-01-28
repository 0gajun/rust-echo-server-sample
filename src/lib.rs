mod server;
mod client;

use client::{Client, Send};

pub fn launch_server(socket_path: &str) {
    server::start_server(socket_path);
}

pub fn launch_client(socket_path: &str) -> Client {
    return client::connect(socket_path);
}

pub fn send_to_server(client: &mut Client, msg: &str) -> usize {
    return client.send(msg);
}

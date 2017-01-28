mod server;
mod client;

pub fn launch_server(socket_path: &str) {
    server::start_server(socket_path);
}

pub fn launch_client(socket_path: &str) {
    client::start_client(socket_path);
}

